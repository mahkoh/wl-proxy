use {
    crate::{
        collector::{CollectorError, collect},
        formatter::{format_interface_file, format_mod_file, format_protocol_file},
    },
    std::{
        fs::File,
        io::{self, BufWriter, Write},
        path::{Path, PathBuf},
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum GeneratorError {
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
                format_protocol_file(f, &protocol)
            })?;
            let dir = generated_dir.join(&*protocol.name);
            create_dir(&dir)?;
            for interface in &protocol.interfaces {
                let file_name = format!("{}.rs", interface.name);
                format_file(&dir.join(&file_name), |f| {
                    format_interface_file(f, &interface)
                })?;
            }
        }
    }

    format_file(&src_dir.join("protocols.rs"), |f| {
        format_mod_file(f, &suits)
    })?;

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
