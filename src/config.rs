use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub projects: Vec<Project>,
    pub log_level: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub folder: String,
    pub remote_name: Option<String>, // defaults to origin
    pub source_branch: String,
    pub target_branch: Option<String>, // defaults to main
}
