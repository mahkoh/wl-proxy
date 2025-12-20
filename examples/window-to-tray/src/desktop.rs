use {
    ini::Ini,
    std::{
        collections::{HashMap, hash_map::Entry},
        env::var,
        path::{Path, PathBuf},
        sync::LazyLock,
    },
};

pub fn get_icon_name(app_id: &str) -> Option<&'static str> {
    APPLICATIONS.get(app_id)?.icon.as_deref()
}

struct Application {
    icon: Option<String>,
}

static APPLICATIONS: LazyLock<HashMap<String, Application>> = LazyLock::new(|| {
    let mut apps = HashMap::new();
    for dir in &*DESKTOP_BASE_DIRS {
        parse_applications_in_dir(&mut apps, dir);
    }
    apps
});

fn parse_applications_in_dir(out: &mut HashMap<String, Application>, dir: &Path) {
    for file in walkdir::WalkDir::new(dir) {
        let Ok(file) = file else {
            continue;
        };
        let Ok(app_id) = file.path().strip_prefix(dir) else {
            continue;
        };
        if app_id.extension() != Some("desktop".as_ref()) {
            continue;
        }
        let app_id = app_id
            .with_extension("")
            .display()
            .to_string()
            .replace("/", "-");
        let entry = match out.entry(app_id) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(v) => v,
        };
        let Ok(application) = std::fs::read_to_string(file.path()) else {
            continue;
        };
        let Ok(mut ini) = Ini::load_from_str(&application) else {
            continue;
        };
        let Some(desc) = ini.delete(Some("Desktop Entry")) else {
            continue;
        };
        let application = Application {
            icon: desc.get("Icon").map(|s| s.to_string()),
        };
        entry.insert(application);
    }
}

static DESKTOP_BASE_DIRS: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    let mut dirs = vec![];
    if let Ok(data_home) = var("XDG_DATA_HOME") {
        dirs.push(format!("{data_home}/applications"));
    } else {
        dirs.push("$HOME/.local/share/applications".to_string());
    }
    if let Ok(data_dirs) = var("XDG_DATA_DIRS") {
        for dir in data_dirs.split(":") {
            dirs.push(format!("{dir}/applications"));
        }
    } else {
        dirs.push("/usr/local/share/applications".to_string());
        dirs.push("/usr/share/applications".to_string());
    }
    dirs.into_iter()
        .flat_map(|d| shellexpand::full(&d).ok().map(|s| s.into_owned()))
        .map(PathBuf::from)
        .collect()
});
