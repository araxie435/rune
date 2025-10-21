mod actions;
mod commands;
mod configs;
mod handler;

fn main() {
    let config: configs::config::Config = configs::config::collect_config();

    handler::handle(config);
}
