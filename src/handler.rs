use crate::actions::install::install::install_handler;
use crate::actions::uninstall::uninstall::uninstall_handler;
use crate::commands::help::help;
use crate::configs::config::Config;
use std::process::exit;

pub fn handle(config: Config) {
    let input: Vec<String> = std::env::args().collect();

    if input.len() < 2 {
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
            exit(0);
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

        "update" => {}

        "upgrade" => {}

        "tree" => {}

        "info" => {}
        
        _ => {}
    }
}
