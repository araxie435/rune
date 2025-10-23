mod actions;
mod commands;
mod configs;
mod handler;

use configs::structures::Config;

fn main() {
    let config: Config = configs::config::collect_config();

    handler::handle(config);
}
