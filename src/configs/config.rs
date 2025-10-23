use std::{path::PathBuf, process::exit};

use crate::configs::structures::{Config, DumpPaths, Manifest, PackageDump, PackagesDump};

pub fn collect_config() -> Config {
    let user: String = std::env::var("USER").unwrap();
    let mut scope: String;

    // If run as root, default to the global scope. Otherwise default to user.
    if user.as_str() != "root" {
        scope = "user".to_string();
    } else {
        scope = "global".to_string();
    }

    return Config {
        user: user.clone(),
        group: user,
        scope: scope,
        is_root: is_root(),
    };
}

fn is_root() -> bool {
    if std::env::var("USER").unwrap_or_default() == "root" {
        return true;
    }
    return false;
}

pub fn collect_manifest(path_to_manifest: &PathBuf) -> Manifest {
    let manifest_content: String =
        std::fs::read_to_string(path_to_manifest).expect("Failed to read manifest file");

    let manifest: Manifest =
        serde_yaml::from_str(&manifest_content).expect("Failed to parse manifest file");
        
    return manifest;
}

pub fn collect_packages_dump(path_to_dump: &PathBuf) -> PackagesDump {
    let dump_content: String =
        std::fs::read_to_string(path_to_dump).expect("Failed to read packages dump file");

    let dump: PackagesDump =
        serde_json::from_str(&dump_content).expect("Failed to parse packages dump file");
        
    return dump;
}

pub fn write_packages_dump(path_to_dump: &PathBuf, dump: &PackagesDump) {
    let dump_content: String =
        serde_json::to_string_pretty(dump).expect("Failed to serialize packages dump");

    std::fs::write(path_to_dump, dump_content).expect("Failed to write packages dump file");
}

pub fn add_package_to_dump(manifest: Manifest, config: &Config) {
    let mut dump: PackagesDump;
    let mut dump_path: PathBuf;

    match config.scope.as_str() {
        "global" => {
            dump_path = DumpPaths::global();
            dump = collect_packages_dump(&dump_path);
        }

        "group" => {
            dump_path = DumpPaths::group(&config.group);
            dump = collect_packages_dump(&dump_path);
        }

        "user" => {
            dump_path = DumpPaths::user(&config.user);
            dump = collect_packages_dump(&dump_path);
        }

        _ => {
            println!("Error in config. Invalid scope {}", config.scope);
            exit(1);
        }
        
    }

    let package_dump: PackageDump = PackageDump {
        version: manifest.version.clone(),
        paths: match config.scope.as_str() {
            "global" => manifest.paths.global.unwrap(),
            "group" => manifest.paths.group.unwrap(),
            "user" => manifest.paths.user.unwrap(),
            _ => {
                println!("Error in config. Invalid scope {}", config.scope);
                exit(1);
            }
        },
    };

    dump.packages.insert(manifest.name.clone(), package_dump);

    write_packages_dump(&dump_path, &dump);
}

pub fn remove_package_from_dump(package: &str, config: &Config) {
    let mut dump: PackagesDump;
    let mut dump_path: PathBuf;

    match config.scope.as_str() {
        "global" => {
            dump_path = DumpPaths::global();
            dump = collect_packages_dump(&dump_path);
        }

        "group" => {
            dump_path = DumpPaths::group(&config.group);
            dump = collect_packages_dump(&dump_path);
        }

        "user" => {
            dump_path = DumpPaths::user(&config.user);
            dump = collect_packages_dump(&dump_path);
        }

        _ => {
            println!("Error in config. Invalid scope {}", config.scope);
            exit(1);
        }
        
    }

    dump.packages.remove(package);

    write_packages_dump(&dump_path, &dump);
}