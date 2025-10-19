use std::{fs, path::{Path, PathBuf}, process::exit};

use crate::{actions::{archive::unarchive_to_path, temp_paths::generate_temp_path}, configs::config::{parse_manifest, Manifest}};

pub fn install_package (path: &str) {
    let temp_path: PathBuf = generate_temp_path();
    unarchive_to_path(&PathBuf::from(path), &temp_path);

    let manifest: Manifest = parse_manifest(&temp_path.join("manifest.yaml"));

    println!("Installing {} - {}", manifest.name, manifest.version);

    // Timely version
    copy_standard_binaries(temp_path.clone());

    std::fs::remove_dir_all(temp_path).unwrap();

    println!("Installed {} - {}", manifest.name, manifest.version);
}

struct InstallPaths;
impl InstallPaths {
        fn global() -> PathBuf {
            PathBuf::from("/bin")
        }

        fn group(group: &str) -> PathBuf {
            PathBuf::from(format!("/usr/local/groups/{group}/bin"))
        }
        
        fn user(user: &str) -> PathBuf {
            PathBuf::from(format!("/home/{user}/.local/bin"))
        }
    }

fn copy_standard_binaries(temp_path: PathBuf) {
    let path_from: PathBuf = temp_path.join("bin");
    let path_to: PathBuf = InstallPaths::global();

    for entry in fs::read_dir(path_from).unwrap() {
        let entry: fs::DirEntry = entry.unwrap();

        if entry.path().is_dir() {
            println!("Invalid package");
            exit(1);
        }

        let file_name: std::ffi::OsString = entry.file_name();
        let dest_path: PathBuf = path_to.join(&file_name);
        fs::copy(entry.path(), dest_path).unwrap();
    }
}
