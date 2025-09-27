use std::process::exit;
use crate::configs::config::Config;
use crate::commands::help;

pub fn handle(config: Config) {
    let input: Vec<String> = std::env::args().collect();

    if input.len() < 2 {
        help::small_help();
        exit(0);
    }


    match input[1].as_str() {
        "help" | "--help" => {
            if input.len() >= 3 {
                help::help(input[2].as_str());
                exit(0);
            }
            help::help("--everything");
            exit(0);
        }
        "install" => {}
        "uninstall" => {}
        "update" => {}
        "upgrade" => {}
        "tree" => {}
        "info" => {}
        _ => {}
    }
}