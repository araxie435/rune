use std::path::PathBuf;

use crate::configs::{config::remove_package_from_dump, structures::{Config, PackageDump}};

pub fn uninstall_package(config: &Config, dump: &PackageDump, package: &str) {
    println!("Uninstalling package {} - {}", package, dump.version);

    for path in &dump.paths.bin {
        let path: PathBuf = PathBuf::from(path);

        if path.exists() {
            std::fs::remove_file(&path).unwrap_or_else(|err| {
                println!("Failed to remove binary {}: {}", path.display(), err);
            });
        }
    }

    remove_package_from_dump(package, config);
}