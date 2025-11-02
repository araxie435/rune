use std::path::PathBuf;
use std::io::Error;

use crate::configs::{config::remove_package_from_dump, structures::{Config, PackageDump}};

pub fn uninstall_package(config: &Config, dump: &PackageDump, package: &str) {
    println!("Uninstalling package {} - {}", package, dump.version);

    if let Some(paths) = &dump.paths.bin {
        for bin_path in paths {
            let expanded = expand_path(bin_path, config);
            let bin_path: PathBuf = PathBuf::from(expanded);
            
            if bin_path.exists() {
                std::fs::remove_file(&bin_path).unwrap_or_else(|err: Error| {
                    println!("Failed to remove binary {}: {}", bin_path.display(), err);
                });
            }
        }
    }

    if let Some(config_list) = &dump.paths.config {
        for config_path in config_list {
            let expanded = expand_path(config_path, config);
            let config_path: PathBuf = PathBuf::from(expanded);
            
            if config_path.exists() {
                if config_path.is_file() {
                    std::fs::remove_file(&config_path).unwrap_or_else(|err: Error| {
                        println!("Failed to remove config file {}: {}", config_path.display(), err);
                    });
                } else {
                    std::fs::remove_dir_all(&config_path).unwrap_or_else(|err: Error| {
                        println!("Failed to remove config dir {}: {}", config_path.display(), err);
                    });
                }
            }
        }
    }

    if let Some(other) = &dump.paths.other {
        for other_path in other {
            let expanded = expand_path(other_path, config);
            let other_path: PathBuf = PathBuf::from(expanded);
            
            if other_path.exists() {
                if other_path.is_file() {
                    std::fs::remove_file(&other_path).unwrap_or_else(|err: Error| {
                        println!("Failed to remove file {}: {}", other_path.display(), err);
                    });
                } else {
                    std::fs::remove_dir_all(&other_path).unwrap_or_else(|err: Error| {
                        println!("Failed to remove dir {}: {}", other_path.display(), err);
                    });
                }
            }
        }
    }

    remove_package_from_dump(package, config);
}

fn expand_path(path: &str, config: &Config) -> String {
    if !path.contains("{}") {
        return path.to_string();
    }

    match config.scope.as_str() {
        "user" => path.replace("{}", &config.user),
        "group" => path.replace("{}", &config.group),
        _ => path.to_string(),
    }
}