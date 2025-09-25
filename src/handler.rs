use std::process::exit;
use crate::configs::config::Config;

pub fn handle(config: Config) {
    let input: Vec<String> = std::env::args().collect();

    if input.len() < 2 {
        // print help
        exit(0)
    }


    match input[1].as_str() {
        "help" | "--help" => {}
        "install" => {}
        "uninstall" => {}
        "update" => {}
        "upgrade" => {}
        "tree" => {}
        "info" => {}
        _ => {}
    }
}