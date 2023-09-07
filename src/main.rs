#[macro_use]
extern crate clap;
use std::{
    env,
    fs::{self, metadata},
    process::{exit, Command, Output, Stdio},
};

use clap::Parser;
use cli::Cli;
use log::{debug, error, info, log_enabled, warn};

mod cli;

fn run_git_fetch() {
    let output = run_command(Command::new("git").arg("fetch"));
    if !output.status.success() {
        error!("Error while fetching repository");
    }
}

fn check_git_branch(branch: String) -> bool {
    let output = run_command(
        Command::new("git")
            .arg("show-ref")
            .arg(format!("refs/remotes/{}", branch)),
    );

    let exists = if !output.status.success() {
        false
    } else {
        let raw_output = String::from_utf8(output.stdout).expect("error getting output");
        !raw_output.is_empty()
    };

    if !exists {
        warn!("Branch {} does not exist", branch);
    }

    return exists;
}

fn run_command(command: &mut Command) -> Output {
    let program = command.get_program().to_str().unwrap().to_string();
    let args = command
        .get_args()
        .into_iter()
        .map(|arg| arg.to_str().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    debug!("Running: \"{} {}\"", program, args);
    let c = command
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!("error running \"{} {}\"", program, args))
        .wait_with_output()
        .expect(&format!(
            "error getting output for \"{} {}\"",
            program, args
        ));

    if log_enabled!(log::Level::Debug) {
        let stdout = c.stdout.clone();

        let output = String::from_utf8(stdout).expect("Error getting output");
        if !output.is_empty() {
            debug!("Output from \"{} {}\" is: \n{}", program, args, output);
        } else {
            debug!("No output from \"{} {}\"", program, args);
        }
        if !c.status.success() {
            let stderr = c.stderr.clone();
            let error = String::from_utf8(stderr).expect("error getting error message");
            if error.is_empty() {
                debug!("No error message from \"{} {}\"", program, args);
            } else {
                debug!("Error from \"{} {}\" is: \n{}", program, args, error);
            }
        }
    }

    return c;
}

fn run_git_status(folder_to_check: String, source_branch: String, target_branch: String) {
    match env::set_current_dir(folder_to_check.to_string()) {
        Ok(_) => {}
        _ => {
            error!("Directory not found {}", folder_to_check);
            return;
        }
    }
    info!("Checking folder {}", folder_to_check);
    run_git_fetch();

    if !check_git_branch(source_branch.to_string()) || !check_git_branch(target_branch.to_string())
    {
        return;
    }
    let output = run_command(
        Command::new("git")
            .arg("diff")
            .arg(source_branch)
            .arg(target_branch),
    );
    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout).expect("error getting output");
        if !raw_output.is_empty() {
            info!(
                "[Merge=yes] There are differences. You need to make a merge on {}",
                folder_to_check.to_string()
            );
        } else {
            info!(
                "[Merge=no] There are no differences on {}",
                folder_to_check.to_string()
            );
        }
    } else {
        let err = String::from_utf8(output.stderr);
        let err_str = err.expect("Error getting error");
        if err_str.contains("not a git repository") {
            warn!(
                "folder {} is not a git repository.",
                folder_to_check.to_string()
            );
        } else {
            error!("Error checking diffs: {}", err_str);
        }
    }
}

fn get_projects_folders(path: String) -> Vec<String> {
    debug!("Getting projects folder");
    let projects_folder = match fs::read_dir(path.to_string()) {
        Ok(folder) => folder,
        _ => {
            error!("Can't read directory {}", path.to_string());
            exit(-1);
        }
    };

    let list_response: Vec<String> = projects_folder
        .into_iter()
        .map(|pf| pf.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    return list_response;
}

fn main() {
    let cli: Cli = Cli::parse();

    if env::var("RUST_LOG").is_err() {
        let level = if cli.debug { "debug" } else { "info" };
        env::set_var("RUST_LOG", level);
    }

    env_logger::init();
    cli.clone().log();

    let projects_dir = cli.projects_folder;
    let source_branch = cli.source_branch;
    let target_branch = cli.target_branch;

    let projects_folders = get_projects_folders(projects_dir.to_string());
    projects_folders
        .into_iter()
        .map(|pf| format!("{}/{}", projects_dir.to_string(), pf))
        .filter(|pf| filter_folders(pf.to_string()))
        .for_each(|pf| run_git_status(pf, source_branch.to_string(), target_branch.to_string()));
}

fn filter_folders(project_folder: String) -> bool {
    match metadata(project_folder.to_string()) {
        Ok(md) => md.is_dir(),
        Err(err) => {
            error!("Error checking if {} is a folder: {}", project_folder, err);
            false
        }
    }
}
