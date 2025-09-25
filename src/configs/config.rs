pub struct Config {
    user: String,
    scope: String
}

pub fn collect_config() -> Config {
    let user = std::env::var("USER").unwrap();

    
    let mut scope: String = "".to_string();

    if user.as_str() != "root" {
        scope = "user".to_string();
    } else {
        scope = "system".to_string();
    }

    return Config { user: user, scope: scope }
}