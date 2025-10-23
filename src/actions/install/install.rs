use crate::actions::download::download_packages;
use crate::actions::install::installer::install_package;
use crate::configs::structures::Config;
use crate::configs::data::get_versions;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub fn install_handler(input: &[String], mut config: Config) {
    let mut packages: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();

    let mut i: usize = 0;
    while i < input.len() {
        let token: &String = &input[i];

        match token.as_str() {
            "--group" => {
                config.scope = "group".to_string();

                if i + 1 >= input.len() {
                    println!("Error. Missing group name after --group");
                    exit(1);
                }

                i += 1;

                if input[i].contains("/") || input[i].contains("~") || input[i].contains(".") {
                    println!("Error. Invalid group name: {}", input[i]);
                    exit(1);
                }

                config.group = input[i].clone();
            }

            "--user" => {
                config.scope = "user".to_string();

                if i + 1 >= input.len() {
                    println!("Error. Missing group name after --user");
                    exit(1);
                }

                i += 1;

                if input[i].contains("/") || input[i].contains("~") || input[i].contains(".") {
                    println!("Error. Invalid user name: {}", input[i]);
                    exit(1);
                }

                config.user = input[i].clone();
            }

            "--global" => {
                config.scope = "global".to_string();
            }

            _ => {
                if !token.contains("/") && !token.contains("~") && !token.contains(".") {
                    packages.push(token.clone());
                    i += 1;
                    continue;
                }

                let path = Path::new(token);

                if !path.exists() {
                    println!("Error. Path {token} doesn't exist");
                    exit(1);
                }
                if !path.is_file() {
                    println!("Error. Path {token} is a directory");
                    exit(1);
                }

                paths.push(token.clone());
            }
        }

        i += 1;
    }

    if !config.is_root {
        println!("Error. Installing packages requires root privileges");
        exit(1);
    }

    install(paths, packages, &config);
}

fn install(mut paths: Vec<String>, packages: Vec<String>, config: &Config) {
    let versions: HashMap<String, String> = get_versions(packages.clone());

    ask_install(&paths, &packages, &versions);

    if !packages.is_empty() {
        let downloaded_paths: Vec<String> = download_packages(packages, versions);

        for path in downloaded_paths {
            paths.push(path);
        }
    }

    for path in paths {
        install_package(&path, &config);
    }

    println!("Installation completed successfully");
    exit(0);
}

fn ask_install(paths: &Vec<String>, packages: &Vec<String>, versions: &HashMap<String, String>) {
    println!("Install packages:");

    for package in packages {
        println!("- {package}\t\t{}", versions.get(package).unwrap());
    }

    println!("Install from paths:");

    for path in paths {
        println!("- {path}");
    }

    print!("Agree? [Y/n] - ");
    std::io::stdout().flush().unwrap();

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();

    let lowered = input.to_lowercase();

    if !(input.is_empty() || lowered == "y" || lowered == "yes") {
        println!("Installation cancelled");
        exit(0);
    }
}
