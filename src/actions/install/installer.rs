use crate::{
    actions::{ archive::unarchive_to_path, temp_paths::generate_temp_path },
    configs::{
        config::{ add_package_to_dump, collect_manifest },
        structures::{ Config, ConfigPaths, InstallPaths, Manifest },
    },
};
use nix::unistd::{ Group, User };
use std::{ fs, io::Error, os::unix::fs::{ PermissionsExt, chown }, path::PathBuf, process::exit };

pub fn install_package(path: &str, config: &Config) {
    let temp_path: PathBuf = generate_temp_path();

    unarchive_to_path(&PathBuf::from(path), &temp_path);

    let manifest: Manifest = collect_manifest(&temp_path.join("manifest.yaml"));

    if !manifest.scopes.iter().any(|s| s == &config.scope) {
        println!("Error. Package {} cannot be installed for scope {}", manifest.name, config.scope);

        println!("Available scopes for this package: {}", manifest.scopes.join(", "));

        exit(1);
    }

    println!("Installing {} - {}", manifest.name, manifest.version);

    copy_standard_files(&temp_path, &config);
    add_package_to_dump(manifest, config);

    std::fs::remove_dir_all(&temp_path).unwrap_or_else(|err: Error| {
        eprintln!("Failed to remove temp path: {err}") // Nothing serious. It will be cleaned itself after reboot, but requires chown&chmod of tempdir
    });
}

fn copy_standard_files(temp_path: &PathBuf, config: &Config) {
    let mut created_paths: Vec<PathBuf> = Vec::new();
    let source_bin: PathBuf = temp_path.join("bin");

    let dest_bin: PathBuf = match config.scope.as_str() {
        "global" => InstallPaths::global(),
        "group" => InstallPaths::group(&config.group),
        "user" => InstallPaths::user(&config.user),
        _ => {
            eprintln!("Error in config. Invalid scope {}", config.scope);
            exit(1);
        }
    };

    if !dest_bin.exists() {
        if let Err(err) = fs::create_dir_all(&dest_bin) {
            eprintln!("Failed to create bin dir {}: {}", dest_bin.display(), err);
            panic_cleanup(&temp_path, &created_paths);
            exit(1);
        }
    }

    if source_bin.exists() {
        created_paths = copy_binaries(&source_bin, &dest_bin, &config, created_paths, &temp_path);
    }

    let source_config: PathBuf = temp_path.join("etc");

    let dest_config: PathBuf = match config.scope.as_str() {
        "global" => ConfigPaths::global(),
        "group" => ConfigPaths::group(&config.group),
        "user" => ConfigPaths::user(&config.user),
        _ => {
            eprintln!("Error in config. Invalid scope {}", config.scope);
            panic_cleanup(&temp_path, &created_paths);
            exit(1);
        }
    };

    if !dest_config.exists() {
        fs::create_dir_all(&dest_config).unwrap_or_else(|err: Error| {
            eprintln!("Failed to create config dir {}: {}", dest_config.display(), err);
            panic_cleanup(&temp_path, &created_paths);
            exit(1);
        });
    }

    if source_config.exists() {
        created_paths = copy_configs_recursive(&source_config, &dest_config, &config, created_paths, &temp_path);
    }
}

fn copy_binaries(
    source: &PathBuf,
    dest: &PathBuf,
    config: &Config,
    mut created_paths: Vec<PathBuf>,
    temp_path: &PathBuf
) -> Vec<PathBuf> {
    let (file_mode, uid, gid): (u32, Option<u32>, Option<u32>) = match config.scope.as_str() {
        "global" => (0o550, Some(0), Some(0)),

        "group" => {
            let gid: u32 = get_group_gid(&config.group).unwrap_or_else(|err| {
                eprintln!("Invalid config: {err}");
                panic_cleanup(&temp_path, &created_paths);
                exit(1);
            });

            (0o550, Some(0), Some(gid))
        }

        "user" => {
            let uid: u32 = get_user_uid(&config.user).unwrap_or_else(|err| {
                eprintln!("Invalid config: {err}");
                panic_cleanup(&temp_path, &created_paths);
                exit(1);
            });

            let gid: u32 = get_user_gid(&config.user).unwrap_or_else(|err| {
                eprintln!("Invalid config: {err}");
                panic_cleanup(&temp_path, &created_paths);
                exit(1);
            });

            (0o550, Some(uid), Some(gid))
        }

        _ => {
            eprintln!("Error in config. Invalid scope {}", config.scope);
            panic_cleanup(&temp_path, &created_paths);
            exit(1);
        }
    };

    for entry_result in fs::read_dir(&source).unwrap_or_else(|err: Error| {
        eprintln!("Cannot read source dir: {err}");
        panic_cleanup(&temp_path, &created_paths);
        exit(1);
    }) {
        let entry = entry_result.unwrap_or_else(|err: Error| {
            eprintln!("Failed reading bin entry {err}");
            panic_cleanup(&temp_path, &created_paths);
            exit(1);
        });

        if entry.path().is_dir() {
            eprintln!("Invalid package: nested directory in bin is not allowed");
            panic_cleanup(&temp_path, &created_paths);
            exit(1);
        }

        let file_name = entry.file_name();
        let dest_file = dest.join(&file_name);

        fs::copy(entry.path(), &dest_file).unwrap_or_else(|err: Error| {
            eprintln!(
                "Failed to copy from {} to {}. Error: {}",
                entry.path().display(),
                dest_file.display(),
                err
            );
            panic_cleanup(&temp_path, &created_paths);
            exit(1);
        });

        created_paths.push(dest_file.clone());

        fs::set_permissions(&dest_file, fs::Permissions::from_mode(file_mode)).unwrap_or_else(
            |err: Error| {
                eprintln!("Failed to set permissions for {}: {}", dest_file.display(), err);
                panic_cleanup(&temp_path, &created_paths);
                exit(1);
            }
        );

        chown(&dest_file, uid, gid).unwrap_or_else(|err: Error| {
            eprintln!("Failed to chown {}: {}", dest_file.display(), err);
            panic_cleanup(&temp_path, &created_paths);
            exit(1);
        });
    }

    return created_paths;
}

fn copy_configs_recursive(
    source: &PathBuf,
    dest: &PathBuf,
    config: &Config,
    mut created_paths: Vec<PathBuf>,
    temp_path: &PathBuf
) -> Vec<PathBuf> {
    let (file_mode, dir_mode, uid, gid): (u32, u32, Option<u32>, Option<u32>) = match config.scope.as_str() {
        "global" => (0o660, 0o660, Some(0), Some(0)),

        "group" => {
            let gid: u32 = get_group_gid(&config.group).unwrap_or_else(|err| {
                eprintln!("Invalid config: {err}");
                panic_cleanup(&temp_path, &created_paths);
                exit(1);
            });

            (0o660, 0o660, Some(0), Some(gid))
        }

        "user" => {
            let uid: u32 = get_user_uid(&config.user).unwrap_or_else(|err| {
                eprintln!("Invalid config: {err}");
                panic_cleanup(&temp_path, &created_paths);
                exit(1);
            });

            let gid: u32 = get_user_gid(&config.user).unwrap_or_else(|err| {
                eprintln!("Invalid config: {err}");
                panic_cleanup(&temp_path, &created_paths);
                exit(1);
            });

            (0o660, 0o660, Some(uid), Some(gid))
        }

        _ => {
            eprintln!("Error in config. Invalid scope {}", config.scope);
            panic_cleanup(&temp_path, &created_paths);
            exit(1);
        }
    };

    for entry_result in fs::read_dir(source).unwrap_or_else(|err: Error| {
        eprintln!("Cannot read dir: {err}");
        panic_cleanup(&temp_path, &created_paths);
        exit(1);
    }) {
        let entry = entry_result.unwrap_or_else(|err: Error| {
            eprintln!("Failed reading bin entry {err}");
            panic_cleanup(&temp_path, &created_paths);
            exit(1);
        });

        let source_path = entry.path();
        let entry_name = entry.file_name();
        let dest_path = dest.join(&entry_name);

        if source_path.is_dir() {
            if !dest_path.exists() {
                fs::create_dir_all(&dest_path).unwrap_or_else(|err: Error| {
                    eprintln!("Failed to create dir {}", &dest_path.display())
                });
                created_paths.push(dest_path.clone());
            }

            fs::set_permissions(&dest_path, fs::Permissions::from_mode(dir_mode)).unwrap_or_else(
                |err: Error| {
                    eprintln!("Failed to set permissions: {err}");
                    panic_cleanup(&temp_path, &created_paths);
                    exit(1);
                }
            );

            chown(&dest_path, uid, gid).unwrap_or_else(|err: Error| {
                eprintln!("Failed to set permissions: {err}");
                panic_cleanup(&temp_path, &created_paths);
                exit(1);
            });

            created_paths = copy_configs_recursive(&source, &dest, &config, created_paths, &temp_path);
        } else {
            fs::copy(&source_path, &dest_path).unwrap_or_else(|err: Error| {
                eprintln!(
                    "Failed to copy from {} to {}. Error: {}",
                    entry.path().display(),
                    dest_path.display(),
                    err
                );
                panic_cleanup(&temp_path, &created_paths);
                exit(1);
            });

            created_paths.push(dest_path.clone());

            fs::set_permissions(&dest_path, fs::Permissions::from_mode(file_mode)).unwrap_or_else(
                |err: Error| {
                    eprintln!("Failed to set permissions: {err}");
                    panic_cleanup(&temp_path, &created_paths);
                    exit(1);
                }
            );

            chown(&dest_path, uid, gid).unwrap_or_else(|err: Error| {
                eprintln!("Failed to set permissions: {err}");
                panic_cleanup(&temp_path, &created_paths);
                exit(1);
            });
        }
    }

    return created_paths;
}

fn panic_cleanup(temp_path: &PathBuf, created_paths: &Vec<PathBuf>) {
    std::fs::remove_dir_all(&temp_path).unwrap_or_else(|err: Error| {
        eprintln!("Failed to remove temp path {}: {}", temp_path.display(), err);
    });
    
    let mut paths: Vec<PathBuf> = created_paths.clone();
    paths.sort_by_key(|path: &PathBuf| std::cmp::Reverse((path.components().count(), path.as_os_str().len())));

    for path in paths {
        if !path.exists(){
            eprintln!("Path {} gone itself", path.display());
            continue;
        }

        if path.is_file() {
            if let Err(err) = std::fs::remove_file(&path) {
                eprintln!("Failed to remove file {}: {}", path.display(), err);
            }
        } else {
            if let Err(err) = std::fs::remove_dir_all(&path) {
                eprintln!("Failed to remove dir {}: {}", path.display(), err);
            }
        }
    }
}

fn get_user_uid(user: &str) -> Result<u32, String> {
    match User::from_name(user) {
        Ok(Some(user)) => Ok(user.uid.as_raw()),
        Ok(None) => Err("User not found".to_string()),
        Err(err) => Err(format!("{err}")),
    }
}

fn get_user_gid(user: &str) -> Result<u32, String> {
    match User::from_name(user) {
        Ok(Some(user)) => Ok(user.gid.as_raw()),
        Ok(None) => Err("User not found".to_string()),
        Err(err) => Err(format!("{err}")),
    }
}

fn get_group_gid(group: &str) -> Result<u32, String> {
    match Group::from_name(group) {
        Ok(Some(group)) => Ok(group.gid.as_raw()),
        Ok(None) => Err("Group not found".to_string()),
        Err(err) => Err(format!("{err}")),
    }
}
