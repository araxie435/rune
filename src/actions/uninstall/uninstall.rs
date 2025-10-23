use std::{path::PathBuf, process::exit};

use crate::{actions::uninstall::uninstaller::uninstall_package, configs::config::{collect_packages_dump, Config, DumpPaths, PackagesDump}};

pub fn uninstall_handler(input: &[String], mut config: Config) {
    let mut packages: Vec<String> = Vec::new();

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

                println!("Error. Invalid package name: {}", token);
                exit(1);
            }
        }

        i += 1;
    }

    if !config.is_root {
        println!("Error. Uninstalling packages requires root privileges.");
        exit(1);
    }

    uninstall(packages, &config);
}

fn uninstall(packages:Vec<String>, config: &Config) {
    let mut dump: PackagesDump;

    match config.scope.as_str() {
        "global" => {
            dump = collect_packages_dump(&DumpPaths::global());
        }

        "group" => {
            dump = collect_packages_dump(&DumpPaths::group(&config.group));
        }

        "user" => {
            dump = collect_packages_dump(&DumpPaths::user(&config.user));
        }

        _ => {
            println!("Error in config. Invalid scope {}", config.scope);
            exit(1);
        }
        
    }

    for package in packages {
        if !dump.packages.contains_key(&package) {
            println!("Package {} isn't installed in {} scope.", package, config.scope);
            exit(1);
        }

        uninstall_package(config, &dump.packages[&package], &package);
    }
}