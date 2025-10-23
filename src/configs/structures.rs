use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};


pub struct Config {
    pub user: String,
    pub group: String,
    pub scope: String,
    pub is_root: bool,
}


pub struct InstallPaths;
impl InstallPaths {
    pub fn global() -> PathBuf {
        PathBuf::from("/bin")
    }

    pub fn group(group: &str) -> PathBuf {
        PathBuf::from(format!("/usr/local/groups/{group}/bin"))
    }

    pub fn user(user: &str) -> PathBuf {
        PathBuf::from(format!("/home/{user}/.local/bin"))
    }
}


#[derive(Deserialize)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub scopes: Vec<String>,
    pub paths: PathScopes,
}


#[derive(Deserialize)]
pub struct PathScopes {
    pub global: Option<PathByUsage>,
    pub group: Option<PathByUsage>,
    pub user: Option<PathByUsage>,
}


#[derive(Deserialize, Serialize)]
pub struct PathByUsage {
    pub bin: Vec<String>,
    pub config: Option<String>,
    pub other: Option<String>,
}


pub struct DumpPaths;
impl DumpPaths {
    pub fn global() -> PathBuf {
        PathBuf::from("/etc/rune/packages_dump.json")
    }

    pub fn group(group: &str) -> PathBuf {
        PathBuf::from(format!("/usr/local/groups/{group}/etc/rune/packages_dump.json"))
    }

    pub fn user(user: &str) -> PathBuf {
        PathBuf::from(format!("/home/{user}/.local/etc/rune/packages_dump.json"))
    }
}


#[derive(Deserialize, Serialize)]
pub struct PackagesDump {
    pub packages: HashMap<String, PackageDump>
}


#[derive(Deserialize, Serialize)]
pub struct PackageDump {
    pub version: String,
    pub paths: PathByUsage,
}