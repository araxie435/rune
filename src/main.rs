mod configs;
mod handler;
mod actions;
mod commands;

fn main() {
    if !is_root() {
        println!("This program must be run as root.");
        println!("*This is temporary solution*");
        std::process::exit(1);
    }

    let config = configs::config::collect_config();

    handler::handle(config);
}

fn is_root() -> bool {
    if std::env::var("USER").unwrap_or_default() == "root" {
        return true;
    }
    return false;
}