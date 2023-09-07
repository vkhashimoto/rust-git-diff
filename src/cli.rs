use clap::arg;
use log::debug;

#[derive(Parser, Clone)]
pub struct Cli {
    #[arg(value_name = "projects-folder", help = "folder with projects to check")]
    pub projects_folder: String,

    #[arg(
        short,
        long,
        value_name = "source branch",
        help = "source branch to check (it must be a remote branch)"
    )]
    pub source_branch: String,

    #[arg(
        short,
        long,
        value_name = "target branch",
        default_value = "origin/main",
        help = "target branch to check (it must be a remote branch)"
    )]
    pub target_branch: String,

    #[arg(long)]
    pub debug: bool,
}

impl Cli {
    pub fn log(self) {
        debug!("Running with the following arguments");
        debug!("    projects_folder = {}", self.projects_folder.to_string());
        debug!("    source_branch = {}", self.source_branch.to_string());
        debug!("    target_branch = {}", self.target_branch.to_string());
        debug!("---");
    }
}
