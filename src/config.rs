use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub projects_folder: String,
    pub projects: Vec<Project>,
    pub merges: Option<bool>,
    pub debug: Option<bool>,
}

#[derive(Deserialize, Clone)]
pub struct Project {
    pub project_folder: String,
    pub source_branch: String,
    pub target_branch: String,
}
