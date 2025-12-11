use {
    isnt::std_1::{
        fs::IsntFileTypeExt,
        primitive::{IsntSliceExt, IsntStrExt},
    },
    std::{path::PathBuf, process::Command},
    walkdir::WalkDir,
};

#[derive(Default)]
struct Repo {
    dir: &'static str,
    subdirs: &'static [&'static str],
    exclude: &'static [&'static str],
}

fn main() {
    let repos = [
        Repo {
            dir: "hyprland-protocols",
            subdirs: &["protocols"],
            ..Default::default()
        },
        Repo {
            dir: "jay-protocols",
            subdirs: &["unstable"],
            ..Default::default()
        },
        Repo {
            dir: "treeland-protocols",
            subdirs: &["xml"],
            ..Default::default()
        },
        Repo {
            dir: "wayland",
            subdirs: &["protocol"],
            exclude: &["tests.xml"],
            ..Default::default()
        },
        Repo {
            dir: "wayland-protocols",
            subdirs: &["stable", "staging", "unstable"],
            exclude: &[
                "linux-dmabuf-unstable-v1.xml",
                "tablet-unstable-v1.xml",
                "tablet-unstable-v2.xml",
                "xdg-foreign-unstable-v1.xml",
                "xdg-shell-unstable-v5.xml",
                "xdg-shell-unstable-v6.xml",
            ],
            ..Default::default()
        },
        Repo {
            dir: "wlr-protocols",
            subdirs: &["unstable"],
            ..Default::default()
        },
    ];
    system("git submodule update --init");
    for repo in repos {
        system(&format!("git -C protocol-repos/{} fetch", repo.dir));
        system(&format!(
            "git -C protocol-repos/{} checkout origin/HEAD",
            repo.dir
        ));
        let mut subdirs = repo.subdirs;
        if subdirs.is_empty() {
            subdirs = &[""];
        }
        let out_dir: PathBuf = format!("protocols/{}", repo.dir).into();
        std::fs::create_dir_all(&out_dir).unwrap();
        for subdir in subdirs {
            let mut path: PathBuf = format!("protocol-repos/{}", repo.dir).into();
            if subdir.is_not_empty() {
                path.push(subdir);
            }
            for file in WalkDir::new(&path) {
                let file = file.unwrap();
                if file.file_type().is_not_file() {
                    continue;
                }
                let name = file.file_name().to_str().unwrap();
                if name.as_bytes().not_ends_with(b".xml") {
                    continue;
                }
                if repo.exclude.contains(&name) {
                    continue;
                }
                let output = out_dir.join(file.file_name());
                eprintln!("cp {} {}", file.path().display(), output.display());
                let contents = std::fs::read(file.path()).unwrap();
                std::fs::write(&output, contents).unwrap();
            }
        }
    }
}

fn system(cmd: &str) {
    eprintln!("{}", cmd);
    let code = Command::new("bash")
        .args(["-c", cmd])
        .status()
        .unwrap()
        .code();
    assert_eq!(code, Some(0));
}
