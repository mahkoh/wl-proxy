use {
    crate::{
        error::Error,
        formatter::{format_interface_file, format_mod_file, format_protocol_file},
        parser::{ParserError, parse},
    },
    std::{
        collections::{HashMap, HashSet},
        fmt::Write as FmtWrite,
        fs::{File, read_to_string},
        io::{self, BufWriter, Write},
        path::{Path, PathBuf},
        rc::Rc,
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
enum BuilderError {
    #[error("Could not read {}", .0.display())]
    ReadFile(PathBuf, #[source] io::Error),
    #[error("Could not open {} for reading", .0.display())]
    OpenDir(PathBuf, #[source] io::Error),
    #[error("Could not read from {}", .0.display())]
    ReadDir(PathBuf, #[source] io::Error),
    #[error("Could not parse {}", .0.display())]
    ParseFile(PathBuf, #[source] ParserError),
    #[error("Could not format {}", .0.display())]
    FormatFile(PathBuf, #[source] io::Error),
    #[error("Could not create {}", .0.display())]
    CreateDir(PathBuf, #[source] io::Error),
    #[error("Could not open {} for writing", .0.display())]
    OpenFile(PathBuf, #[source] io::Error),
}

/// A builder for `wl-client` wrappers.
pub struct Builder {
    files: Vec<(Rc<String>, PathBuf)>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            files: Default::default(),
        }
    }
}

impl Builder {
    /// Generates the code.
    pub fn build(self) -> Result<(), Error> {
        self.build_().map_err(|e| Error(Box::new(e)))
    }

    fn build_(mut self) -> Result<(), BuilderError> {
        let mut root_dir = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        root_dir.pop();

        let wl_proxy_dir = root_dir.join("wl-proxy");
        let src_dir = wl_proxy_dir.join("src");
        let generated_dir = src_dir.join("protocols");
        let suite = [
            "hyprland-protocols",
            "jay-protocols",
            "other",
            "treeland-protocols",
            "wayland",
            "wayland-protocols",
            "wlr-protocols",
        ];
        let protocols_dir = root_dir.join("protocols");
        let _ = std::fs::remove_dir_all(&generated_dir);
        create_dir(&generated_dir)?;

        let mut protocol_objects = vec![];

        for suite in suite {
            let dir = protocols_dir.join(suite);
            let suite = Rc::new(suite.to_string());
            let iter = match std::fs::read_dir(&dir) {
                Ok(c) => c,
                Err(e) => return Err(BuilderError::OpenDir(dir, e)),
            };
            for file in iter {
                let file = match file {
                    Ok(f) => f,
                    Err(e) => return Err(BuilderError::ReadDir(dir, e)),
                };
                if !file.file_name().as_encoded_bytes().ends_with(b".xml") {
                    continue;
                }
                self.files.push((suite.clone(), file.path()));
            }
        }
        self.files.sort();
        let mut interface_to_protocol = HashMap::new();
        let mut protocol_interface_dependencies = HashMap::<Rc<String>, HashSet<Rc<String>>>::new();
        let mut suite_to_protocol = HashMap::<Rc<String>, HashSet<Rc<String>>>::new();
        let mut all_protocols = Vec::new();
        for (suite, file) in self.files {
            let contents = match std::fs::read(&file) {
                Ok(c) => c,
                Err(e) => return Err(BuilderError::ReadFile(file, e)),
            };
            let mut protocols = match parse(&contents) {
                Ok(c) => c,
                Err(e) => return Err(BuilderError::ParseFile(file, e)),
            };
            protocols.sort_by(|p1, p2| p1.name.cmp(&p2.name));
            for mut protocol in protocols {
                if *suite != "wayland" {
                    all_protocols.push(protocol.name.clone());
                    suite_to_protocol
                        .entry(suite.clone())
                        .or_default()
                        .insert(protocol.name.clone());
                }
                protocol.interfaces.sort_by(|i1, i2| i1.name.cmp(&i2.name));
                let protocol_file = format!("{}.rs", protocol.name);
                format_file(&generated_dir.join(&protocol_file), |f| {
                    format_protocol_file(f, &protocol)
                })?;
                let dir = generated_dir.join(&*protocol.name);
                create_dir(&dir)?;
                let mut interfaces = vec![];
                for interface in protocol.interfaces {
                    interface_to_protocol.insert(interface.name.clone(), protocol.name.clone());
                    for msg in &interface.messages {
                        for arg in &msg.args {
                            let mut interface = arg.interface.clone();
                            if interface.is_none()
                                && let Some(enum_) = &arg.enum_
                                && let Some((i, _)) = enum_.split_once('.')
                            {
                                interface = Some(Rc::new(i.to_string()));
                            }
                            if let Some(interface) = interface {
                                protocol_interface_dependencies
                                    .entry(protocol.name.clone())
                                    .or_default()
                                    .insert(interface);
                            }
                        }
                    }
                    let file_name = format!("{}.rs", interface.name);
                    format_file(&dir.join(&file_name), |f| {
                        format_interface_file(f, &interface)
                    })?;
                    let mut enums = vec![];
                    for enum_ in interface.enums {
                        enums.push(enum_.name);
                    }
                    interfaces.push((interface.name, enums, interface.version));
                }
                protocol_objects.push((protocol.name, interfaces));
            }
        }

        format_file(&src_dir.join("protocols.rs"), |f| {
            format_mod_file(f, &protocol_objects)
        })?;

        {
            let mut suite_protocols: Vec<_> = suite_to_protocol
                .into_iter()
                .map(|(suite, protocols)| {
                    let mut protocols: Vec<_> = protocols.into_iter().collect();
                    protocols.sort();
                    (suite, protocols)
                })
                .collect();
            suite_protocols.sort_by(|l, r| l.0.cmp(&r.0));
            let mut protocol_dependencies = HashMap::new();
            for (protocol, interfaces) in protocol_interface_dependencies {
                let mut deps = HashSet::new();
                for interface in interfaces {
                    let dep = interface_to_protocol.get(&interface).unwrap();
                    if dep != &protocol && **dep != "wayland" {
                        deps.insert(dep);
                    }
                }
                let mut deps: Vec<_> = deps.into_iter().cloned().collect();
                deps.sort();
                if !deps.is_empty() {
                    protocol_dependencies.insert(protocol, deps);
                }
            }
            all_protocols.sort();
            let cargo_toml_path = wl_proxy_dir.join("Cargo.toml");
            let old = match read_to_string(&cargo_toml_path) {
                Ok(o) => o,
                Err(e) => {
                    return Err(BuilderError::ReadFile(cargo_toml_path, e));
                }
            };
            let mut new = String::new();
            let generated_start = "# --generated start--\n";
            let generated_end = "# --generated end--\n";
            let start = old.find(generated_start).unwrap();
            let end = old.find(generated_end).unwrap();
            new.push_str(&old[..start]);
            new.push_str(generated_start);
            writeln!(new, "all-protocols = [").unwrap();
            for (suite, _) in &suite_protocols {
                writeln!(new, r#"    "suite-{suite}","#).unwrap();
            }
            writeln!(new, "]").unwrap();
            writeln!(new).unwrap();
            for (suite, protocols) in &suite_protocols {
                writeln!(new, "suite-{suite} = [").unwrap();
                for protocol in protocols {
                    writeln!(new, r#"    "protocol-{protocol}","#).unwrap();
                }
                writeln!(new, "]").unwrap();
            }
            writeln!(new).unwrap();
            for protocol in all_protocols {
                write!(new, "protocol-{protocol} = [").unwrap();
                if let Some(deps) = protocol_dependencies.get(&protocol) {
                    for (idx, dep) in deps.iter().enumerate() {
                        if idx > 0 {
                            write!(new, ", ").unwrap();
                        }
                        write!(new, r#""protocol-{dep}""#).unwrap();
                    }
                }
                writeln!(new, "]").unwrap();
            }
            new.push_str(&old[end..]);
            std::fs::write(&cargo_toml_path, new).unwrap();
        }
        Ok(())
    }
}

fn create_dir(path: &Path) -> Result<(), BuilderError> {
    if let Err(e) = std::fs::create_dir_all(path) {
        return Err(BuilderError::CreateDir(path.to_owned(), e));
    }
    Ok(())
}

fn open_file(path: &Path) -> Result<BufWriter<File>, BuilderError> {
    let file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path);
    match file {
        Ok(f) => Ok(BufWriter::new(f)),
        Err(e) => Err(BuilderError::OpenFile(path.to_owned(), e)),
    }
}

fn format_file(
    path: &Path,
    f: impl FnOnce(&mut BufWriter<File>) -> io::Result<()>,
) -> Result<(), BuilderError> {
    let mut file = open_file(path)?;
    let mut res = f(&mut file);
    if res.is_ok() {
        res = file.flush();
    }
    if let Err(e) = res {
        return Err(BuilderError::FormatFile(path.to_owned(), e));
    }
    Ok(())
}
