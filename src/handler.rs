use crate::actions::install::install::install_handler;
use crate::actions::uninstall::uninstall::uninstall_handler;
use crate::commands::help::help;
use crate::configs::structures::Config;
use std::process::exit;

pub fn handle(config: Config) {
    let input: Vec<String> = std::env::args().collect();

    if input.len() < 2 {
        println!("Not enough arguments!");
        help("--everything");
        exit(0);
    }

    match input[1].as_str() {
        "help" | "--help" => {
            if input.len() >= 3 {
                help(input[2].as_str());
                exit(0);
            }
            help("--everything");
        }

        "install" => {
            if input.len() >= 3 {
                install_handler(&input[2..], config);
                exit(0);
            }
            help("install")
        }

        "uninstall" => {
            if input.len() >= 3 {
                uninstall_handler(&input[2..], config);
                exit(0);
            }
            help("uninstall");
        }

        "update" => {
            println!("update feature will be added later");
        }

        "upgrade" => {
            println!("upgrade feature will be added later");
        }

        "tree" => {
            println!("tree feature will be added later");
        }

        "info" => {
            println!("info feature will be added later");
        }
        
        input => {
            println!("Unknown command: {input}");
            help("--everything");
        }
    }
}
