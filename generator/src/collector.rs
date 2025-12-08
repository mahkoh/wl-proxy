use {
    crate::{
        ast::Protocol,
        parser::{ParserError, parse},
    },
    std::{
        io,
        os::unix::ffi::OsStrExt,
        path::{Path, PathBuf},
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("Could not read {}", .0.display())]
    ReadFile(PathBuf, #[source] io::Error),
    #[error("Could not open {} for reading", .0.display())]
    OpenDir(PathBuf, #[source] io::Error),
    #[error("Could not read from {}", .0.display())]
    ReadDir(PathBuf, #[source] io::Error),
    #[error("Could not parse {}", .0.display())]
    ParseFile(PathBuf, #[source] ParserError),
}

pub(crate) struct Suite {
    pub(crate) name: &'static str,
    pub(crate) is_wayland: bool,
    pub(crate) protocols: Vec<Protocol>,
}

pub(crate) fn collect() -> Result<Vec<Suite>, CollectorError> {
    let mut root_dir = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    root_dir.pop();
    let suite_names = [
        "hyprland-protocols",
        "jay-protocols",
        "external",
        // "treeland-protocols",
        "wayland",
        "wayland-protocols",
        "wlr-protocols",
        "wlproxy",
        "river-protocols",
        "weston-protocols",
        "cosmic-protocols",
    ];
    let protocols_dir = root_dir.join("protocols");
    let mut suits = vec![];
    for suite in suite_names {
        let dir = protocols_dir.join(suite);
        let iter = match std::fs::read_dir(&dir) {
            Ok(c) => c,
            Err(e) => return Err(CollectorError::OpenDir(dir, e)),
        };
        let mut protocols = vec![];
        for file in iter {
            let file = match file {
                Ok(f) => f,
                Err(e) => return Err(CollectorError::ReadDir(dir, e)),
            };
            if !file.file_name().as_bytes().ends_with(b".xml") {
                continue;
            }
            let file = file.path();
            let contents = match std::fs::read(&file) {
                Ok(c) => c,
                Err(e) => return Err(CollectorError::ReadFile(file, e)),
            };
            let p = match parse(&contents) {
                Ok(c) => c,
                Err(e) => return Err(CollectorError::ParseFile(file, e)),
            };
            protocols.extend(p);
        }
        protocols.sort_by(|p1, p2| p1.name.cmp(&p2.name));
        for protocol in &mut protocols {
            protocol.interfaces.sort_by(|i1, i2| i1.name.cmp(&i2.name));
        }
        suits.push(Suite {
            name: suite,
            is_wayland: suite == "wayland",
            protocols,
        });
    }
    Ok(suits)
}
