use log::{error, info};
use rust_git_diff::command::{CommandResult, CommandRunner, DefaultCommandRunner};
use std::{env, fs};
use toml::de::Error;
extern crate clap;
extern crate serde;

use rust_git_diff::git::{Git, Repo};

use crate::config::Config;

mod command;
mod config;

fn main() {
    //TODO: Read from default folder or args
    let config_file_content = fs::read_to_string("config.toml");
    let config_file: Result<Config, Error> =
        toml::from_str(&config_file_content.unwrap_or("".to_string()));

    let config = if config_file.is_ok() {
        config_file.unwrap()
    } else {
        panic!("Could not read 'config.toml'");
    };

    env::set_var(
        "RUST_LOG",
        match config.log_level.as_ref() {
            Some(level) => level.as_str(),
            None => "info",
        },
    );
    env_logger::init();

    let mut git = Git::new();
    config
        .projects
        .iter()
        .map(|project| {
            Repo::new(
                project.name.as_str(),
                project.folder.as_str(),
                match project.remote_name.as_ref() {
                    Some(remote_name) => remote_name.as_str(),
                    None => "origin",
                },
                project.source_branch.as_str(),
                match project.target_branch.as_ref() {
                    Some(remote_name) => remote_name.as_str(),
                    None => "main",
                },
            )
        })
        .for_each(|repository| {
            git.add_repository(repository);
        });

    git.repositories.iter().for_each(process_repository);

    info!("Checked all repositories");
}

fn process_repository(repository: &Repo) {
    let runner = DefaultCommandRunner {};
    let fetch_result = repository
        .fetch()
        .iter()
        .map(|command| runner.run(command))
        .collect::<Vec<CommandResult>>();

    let fetch_error = fetch_result.iter().filter(|result| !result.0).next();
    match fetch_error {
        Some(error) => {
            error!("{}", error.2.as_ref().unwrap());
            return;
        }
        _ => {}
    };

    let diff_result = runner.run(&repository.diff());

    if diff_result.0 {
        match diff_result.1 {
            None => info!("Success checking repository, but no output is avaiable."),
            Some(output) => {
                if output.is_empty() {
                    info!("There are no differences in '{}'", repository.path);
                } else {
                    info!(
                        "There are differences between {}/{} and {}/{} in '{}'",
                        repository.remote_name,
                        repository.source_branch,
                        repository.remote_name,
                        repository.target_branch,
                        repository.name
                    );
                }
            }
        }
    } else {
        error!("Error checking repository {}", repository.path);
        match diff_result.2 {
            None => error!("No error message available."),
            Some(message) => error!("Error message: {}", message),
        }
    }
}
