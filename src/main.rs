mod configs;
mod handler;
mod actions;
mod commands;

fn main() {
    let config = configs::config::collect_config();

    handler::handle(config);
}
