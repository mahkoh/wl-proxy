use std::path::PathBuf;
use std::process::Command;
use isnt::std_1::fs::IsntFileTypeExt;
use isnt::std_1::primitive::{IsntSliceExt, IsntStrExt};
use walkdir::WalkDir;

fn main() {
    let repos = [
        Repo {
            dir: "hyprland-protocols",
            subdirs: &["protocols"],
        },
    ];
    system("git submodule update --init");
    for repo in repos {
        system(&format!("git -C protocol-repos/{} fetch", repo.dir));
        system(&format!("git -C protocol-repos/{} checkout origin/HEAD", repo.dir));
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
                if file.file_name().as_encoded_bytes().not_ends_with(b".xml") {
                    continue;
                }
                let contents = std::fs::read(file.path()).unwrap();
                let output = out_dir.join(file.file_name());
                std::fs::write(&output, contents).unwrap();
            }
        }
    }
}

#[derive(Default)]
struct Repo {
    dir: &'static str,
    subdirs: &'static [&'static str],
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
