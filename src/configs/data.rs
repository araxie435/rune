use std::{collections::HashMap, fs, process::exit};
use serde_json::from_str;

pub fn get_versions(packages: Vec<String>) -> HashMap<String, String> {
    let mut versions: HashMap<String, String> = HashMap::new();

    let package_db_file: &String = &fs::read_to_string("/etc/rune/package-db.json").unwrap();
    let package_db: HashMap<String, String> = from_str::<HashMap<String, String>>(&package_db_file).unwrap();

    for package in packages {
        if !package_db.contains_key(&package) {
            println!("Package {package} not found!");
            exit(1);
        }

        versions.insert(package.clone(), package_db.get(&package).unwrap().to_owned());
    }

    return versions;
}