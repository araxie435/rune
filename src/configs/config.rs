use std::path::PathBuf;

pub struct Config {
    pub user: String,
    pub group: String,
    pub scope: String,
}

#[derive(serde::Deserialize)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub scopes: Vec<String>,
    pub paths: PathScopes,
}

#[derive(serde::Deserialize)]
pub struct PathScopes {
    pub global: Option<PathByUsage>,
    pub group: Option<PathByUsage>,
    pub user: Option<PathByUsage>,
}

#[derive(serde::Deserialize)]
pub struct PathByUsage {
    pub bin: String,
    pub config: Option<String>,
    pub other: Option<String>,
}

pub fn collect_config() -> Config {
    let user: String = std::env::var("USER").unwrap();
    let mut scope: String;

    // If run as root, default to the global scope. Otherwise default to user.
    if user.as_str() != "root" {
        scope = "user".to_string();
    } else {
        scope = "global".to_string();
    }

    return Config {
        user: user.clone(),
        group: user,
        scope: scope,
    };
}

pub fn parse_manifest(path_to_manifest: &PathBuf) -> Manifest {
    let manifest_content: String =
        std::fs::read_to_string(path_to_manifest).expect("Failed to read manifest file");

    let manifest: Manifest =
        serde_yaml::from_str(&manifest_content).expect("Failed to parse manifest file");
        
    return manifest;
}
