use {
    crate::{
        collector::{CollectorError, collect},
        formatter::{
            format_baseline_file, format_baseline_txt, format_interface_file, format_mod_file,
            format_protocol_file,
        },
    },
    std::{
        collections::HashMap,
        fmt::Write as FmtWrite,
        fs::{File, read_to_string},
        io::{self, BufWriter, Write},
        path::{Path, PathBuf},
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("could not read {}", .0.display())]
    ReadFile(PathBuf, #[source] io::Error),
    #[error("could not wrie {}", .0.display())]
    WriteFile(PathBuf, #[source] io::Error),
    #[error("could not format {}", .0.display())]
    FormatFile(PathBuf, #[source] io::Error),
    #[error("could not create {}", .0.display())]
    CreateDir(PathBuf, #[source] io::Error),
    #[error("could not open {} for writing", .0.display())]
    OpenFile(PathBuf, #[source] io::Error),
    #[error("could not collect protocols")]
    Collect(#[from] CollectorError),
}

pub fn main() -> Result<(), GeneratorError> {
    let suits = collect()?;
    let mut root_dir = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    root_dir.pop();

    let wl_proxy_dir = root_dir.join("wl-proxy");
    let src_dir = wl_proxy_dir.join("src");

    let generated_dir = src_dir.join("protocols");
    let _ = std::fs::remove_dir_all(&generated_dir);
    create_dir(&generated_dir)?;
    for suite in &suits {
        for protocol in &suite.protocols {
            let protocol_file = format!("{}.rs", protocol.name);
            format_file(&generated_dir.join(&protocol_file), |f| {
                format_protocol_file(f, protocol)
            })?;
            let dir = generated_dir.join(&*protocol.name);
            create_dir(&dir)?;
            for interface in &protocol.interfaces {
                let file_name = format!("{}.rs", interface.name);
                format_file(&dir.join(&file_name), |f| {
                    format_interface_file(f, interface)
                })?;
            }
        }
    }

    format_file(&src_dir.join("protocols.rs"), |f| {
        format_mod_file(f, &suits)
    })?;

    format_file(&src_dir.join("baseline/versions/prototyping.rs"), |f| {
        format_baseline_file(f, &suits)
    })?;

    format_file(&src_dir.join("baseline/versions/prototyping.txt"), |f| {
        format_baseline_txt(f, &suits)
    })?;

    {
        let mut interface_to_protocol = HashMap::new();
        for protocol in suits.iter().flat_map(|s| s.protocols.iter()) {
            for interface in &protocol.interfaces {
                interface_to_protocol.insert(&*interface.name, protocol);
            }
        }
        let mut protocol_dependencies = HashMap::new();
        for protocol in suits.iter().flat_map(|s| s.protocols.iter()) {
            let mut deps = HashMap::new();
            for arg in protocol
                .interfaces
                .iter()
                .flat_map(|i| i.messages.iter().flat_map(|m| m.args.iter()))
            {
                let mut interface = arg.interface.as_deref();
                if interface.is_none()
                    && let Some(e) = &arg.enum_
                    && let Some((i, _)) = e.split_once('.')
                {
                    interface = Some(i);
                }
                if let Some(i) = interface {
                    let dep = interface_to_protocol.get(i).unwrap();
                    if dep.name != protocol.name && !dep.is_wayland {
                        deps.insert(&dep.name, *dep);
                    }
                }
            }
            let mut deps: Vec<_> = deps.values().copied().collect();
            deps.sort_by_key(|d| &d.name);
            if !deps.is_empty() {
                protocol_dependencies.insert(&protocol.name, deps);
            }
        }
        let cargo_toml_path = wl_proxy_dir.join("Cargo.toml");
        let old = match read_to_string(&cargo_toml_path) {
            Ok(o) => o,
            Err(e) => {
                return Err(GeneratorError::ReadFile(cargo_toml_path, e));
            }
        };
        let mut new = String::new();
        let generated_start = "# --generated start--\n";
        let generated_end = "# --generated end--\n";
        let start = old.find(generated_start).unwrap();
        let end = old.find(generated_end).unwrap();
        new.push_str(&old[..start]);
        new.push_str(generated_start);
        let _ = writeln!(new, "all-protocols = [");
        for suite in &suits {
            if suite.is_wayland {
                continue;
            }
            let _ = writeln!(new, r#"    "suite-{}","#, suite.name);
        }
        let _ = writeln!(new, "]");
        let _ = writeln!(new);
        for suite in &suits {
            if suite.is_wayland {
                continue;
            }
            let _ = writeln!(new, "suite-{} = [", suite.name);
            for protocol in &suite.protocols {
                if protocol.is_wlproxy_test {
                    continue;
                }
                let _ = writeln!(new, r#"    "protocol-{}","#, protocol.name);
            }
            let _ = writeln!(new, "]");
        }
        let _ = writeln!(new);
        for protocol in suits.iter().flat_map(|s| s.protocols.iter()) {
            if protocol.is_wayland || protocol.is_wlproxy_test {
                continue;
            }
            let _ = write!(new, "protocol-{} = [", protocol.name);
            if let Some(deps) = protocol_dependencies.get(&protocol.name) {
                for (idx, dep) in deps.iter().enumerate() {
                    if idx > 0 {
                        let _ = write!(new, ", ");
                    }
                    let _ = write!(new, r#""protocol-{}""#, dep.name);
                }
            }
            let _ = writeln!(new, "]");
        }
        new.push_str(&old[end..]);
        std::fs::write(&cargo_toml_path, new)
            .map_err(|e| GeneratorError::WriteFile(cargo_toml_path, e))?;
    }

    Ok(())
}

fn create_dir(path: &Path) -> Result<(), GeneratorError> {
    if let Err(e) = std::fs::create_dir_all(path) {
        return Err(GeneratorError::CreateDir(path.to_owned(), e));
    }
    Ok(())
}

fn open_file(path: &Path) -> Result<BufWriter<File>, GeneratorError> {
    let file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path);
    match file {
        Ok(f) => Ok(BufWriter::new(f)),
        Err(e) => Err(GeneratorError::OpenFile(path.to_owned(), e)),
    }
}

fn format_file(
    path: &Path,
    f: impl FnOnce(&mut BufWriter<File>) -> io::Result<()>,
) -> Result<(), GeneratorError> {
    let mut file = open_file(path)?;
    let mut res = f(&mut file);
    if res.is_ok() {
        res = file.flush();
    }
    if let Err(e) = res {
        return Err(GeneratorError::FormatFile(path.to_owned(), e));
    }
    Ok(())
}
