use std::io::Write;
use std::path::Path;
use std::collections::HashMap;
use std::process::exit;
use crate::actions::download::download_packages;
use crate::actions::install::installer::install_package;
use crate::configs::data::get_versions;

pub fn install_handler(input: &[String]) {
    let mut packages: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();

    for input in input {
        if !input.contains("/") && !input.contains("~") && !input.contains(".") {
            packages.push(input.clone());
            continue;
        }

        let path = Path::new(input);

        if !path.exists() {
            println!("Error. Path {input} doesn't exist");
            exit(1);
        }
        if !path.is_file() {
            println!("Error. Path {input} is a directory");
            exit(1);
        }

        paths.push(input.clone());
    }

    install(paths,packages);
}

fn install(mut paths: Vec<String>, packages: Vec<String>) {

    let versions: HashMap<String, String> = get_versions(packages.clone());

    ask_install(paths.clone(), packages.clone(), versions.clone());

    let mut downloaded_paths: Vec<String> = Vec::new();

    if !packages.is_empty() {
        downloaded_paths = download_packages(packages, versions);
    }

    for path in downloaded_paths {
        paths.push(path);
    }

    for path in paths {
        install_package(&path);
    }

    println!("Installation completed successfully");
    exit(0);
}

fn ask_install(paths: Vec<String>, packages: Vec<String>, versions: HashMap<String, String>) {
    println!("Install packages:");

    for package in packages {
        println!("- {package}\t\t{}", versions.get(&package).unwrap());
    }
    
    println!("Install from paths:");

    for path in paths {
        println!("- {path}");
    }

    print!("Agree? [Y/n] - ");
    std::io::stdout().flush().unwrap();

    let mut input: String= String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();

    if !input.contains("") && !input.contains("y") && !input.contains("Y") {
        exit(0);
    }
}