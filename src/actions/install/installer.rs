use nix::unistd::{Group, User};
use std::{
    fs,
    os::unix::fs::{PermissionsExt, chown},
    path::PathBuf,
    process::exit,
};

use crate::{
    actions::{archive::unarchive_to_path, temp_paths::generate_temp_path},
    configs::config::{add_package_to_dump, collect_manifest, Config, InstallPaths, Manifest},
};

pub fn install_package(path: &str, config: &Config) {
    let temp_path: PathBuf = generate_temp_path();

    unarchive_to_path(&PathBuf::from(path), &temp_path);

    let manifest: Manifest = collect_manifest(&temp_path.join("manifest.yaml"));

    if !manifest.scopes.iter().any(|s| s == &config.scope) {
        println!(
            "Error. Package {} cannot be installed for scope {}",
            manifest.name, config.scope
        );

        println!(
            "Available scopes for this package: {}",
            manifest.scopes.join(", ")
        );

        exit(1);
    }

    println!("Installing {} - {}", manifest.name, manifest.version);

    // Timely version
    copy_standard_binaries(&temp_path, &config);
    add_package_to_dump(manifest, config);

    std::fs::remove_dir_all(&temp_path).unwrap();
}

fn copy_standard_binaries(temp_path: &PathBuf, config: &Config) {
    let path_from: PathBuf = temp_path.join("bin");

    let path_to: PathBuf;
    match config.scope.as_str() {
        "global" => {
            path_to = InstallPaths::global();
        }
        "group" => {
            path_to = InstallPaths::group(&config.group);
        }
        "user" => {
            path_to = InstallPaths::user(&config.user);
        }
        _ => {
            println!("Error in config. Invalid scope {}", config.scope);
            exit(1);
        }
    }

    for entry in fs::read_dir(path_from).unwrap() {
        let entry: fs::DirEntry = entry.unwrap();

        if entry.path().is_dir() {
            println!("Invalid package");
            exit(1);
        }

        let file_name: std::ffi::OsString = entry.file_name();
        let dest_path: PathBuf = path_to.join(&file_name);

        if !path_to.exists() {
            fs::create_dir_all(&path_to).unwrap();
        }

        fs::copy(entry.path(), &dest_path).unwrap();

        match config.scope.as_str() {
            "global" => {
                fs::set_permissions(&dest_path, fs::Permissions::from_mode(0o775)).unwrap_or_else(|err| {
                    eprintln!("Failed to set permissions for {}: {}", dest_path.display(), err);
                    panic_cleanup(&temp_path, &dest_path);
                    exit(1);
                });
            }

            "group" => {
                fs::set_permissions(&dest_path, fs::Permissions::from_mode(0o750)).unwrap_or_else(|err| {
                    eprintln!("Failed to set permissions for {}: {}", dest_path.display(), err);
                    panic_cleanup(&temp_path, &dest_path);
                    exit(1);
                });
                let gid: u32 = get_group_gid(&config.group);
                chown(&dest_path, Some(0), Some(gid)).unwrap_or_else(|err| {
                    eprintln!("Failed to chown {}: {}", dest_path.display(), err);
                    exit(1);
                });
            }

            "user" => {
                fs::set_permissions(&dest_path, fs::Permissions::from_mode(0o770)).unwrap_or_else(|err| {
                    eprintln!("Failed to set permissions for {}: {}", dest_path.display(), err);
                    panic_cleanup(&temp_path, &dest_path);
                    exit(1);
                });

                let uid: u32 = get_user_uid(&config.user);
                let gid: u32 = get_user_gid(&config.user);
                chown(&dest_path, Some(uid), Some(gid)).unwrap_or_else(|err| {
                    eprintln!("Failed to chown {}: {}", dest_path.display(), err);
                    exit(1);
                });
            }

            _ => {} // This case is already handled above
        }
    }
}

fn panic_cleanup(temp_path: &PathBuf, dest_path: &PathBuf) {
    eprintln!("An error occurred. Cleaning up...");
    std::fs::remove_dir_all(&temp_path).unwrap();
    std::fs::remove_file(&dest_path).unwrap();
}

fn get_user_uid(user: &str) -> u32 {
    match User::from_name(user) {
        Ok(Some(u)) => u.uid.as_raw(),

        Ok(None) => {
            eprintln!("Error: user '{}' not found", user);
            exit(1);
        }

        Err(err) => {
            eprintln!("Error looking up user '{}': {}", user, err);
            exit(1);
        }
    }
}

fn get_user_gid(user: &str) -> u32 {
    match User::from_name(user) {
        Ok(Some(u)) => u.gid.as_raw(),

        Ok(None) => {
            eprintln!("Error: user '{}' not found", user);
            exit(1);
        }

        Err(err) => {
            eprintln!("Error looking up user '{}': {}", user, err);
            exit(1);
        }
    }
}

fn get_group_gid(group: &str) -> u32 {
    match Group::from_name(group) {
        Ok(Some(g)) => g.gid.as_raw(),

        Ok(None) => {
            eprintln!("Error: group '{}' not found", group);
            exit(1);
        }

        Err(err) => {
            eprintln!("Error looking up group '{}': {}", group, err);
            exit(1);
        }
    }
}
