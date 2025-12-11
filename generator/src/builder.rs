use {
    crate::{
        error::Error,
        formatter::{format_interface_file, format_mod_file, format_protocol_file},
        parser::{ParserError, parse},
    },
    std::{
        fs::File,
        io::{self, BufWriter, Write},
        path::{Path, PathBuf},
        sync::Arc,
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
    files: Vec<ProtocolFile>,
}

struct ProtocolFile {
    feature: Option<Arc<String>>,
    path: PathBuf,
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

        let src_dir = root_dir.join("wl-proxy/src");
        let generated_dir = src_dir.join("generated");
        let dirs = [
            ("core", None),
            ("wayland-protocols", Some("wayland-protocols")),
            ("wlr", Some("wlr-protocols")),
        ];
        let protocols_dir = root_dir.join("protocols");
        std::fs::remove_dir_all(&generated_dir).unwrap();
        create_dir(&generated_dir)?;

        let mut protocol_objects = vec![];

        for (dir, feature) in dirs {
            let dir = protocols_dir.join(dir);
            let feature = feature.map(|f| Arc::new(f.to_string()));
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
                self.files.push(ProtocolFile {
                    feature: feature.clone(),
                    path: file.path(),
                });
            }
        }
        self.files.sort_by(|f1, f2| f1.path.cmp(&f2.path));
        for file in self.files {
            let contents = match std::fs::read(&file.path) {
                Ok(c) => c,
                Err(e) => return Err(BuilderError::ReadFile(file.path, e)),
            };
            let mut protocols = match parse(&contents) {
                Ok(c) => c,
                Err(e) => return Err(BuilderError::ParseFile(file.path, e)),
            };
            protocols.sort_by(|p1, p2| p1.name.cmp(&p2.name));
            for mut protocol in protocols {
                protocol.interfaces.sort_by(|i1, i2| i1.name.cmp(&i2.name));
                let protocol_file = format!("{}.rs", protocol.name);
                format_file(&generated_dir.join(&protocol_file), |f| {
                    format_protocol_file(f, &protocol)
                })?;
                let dir = generated_dir.join(&protocol.name);
                create_dir(&dir)?;
                let mut interfaces = vec![];
                for interface in protocol.interfaces {
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
                protocol_objects.push((protocol.name, file.feature.clone(), interfaces));
            }
        }

        format_file(&src_dir.join("generated.rs"), |f| {
            format_mod_file(f, &protocol_objects)
        })?;
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
