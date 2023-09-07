use std::{
    env,
    fs::{self, metadata},
    process::{exit, Command, Output, Stdio},
};

use clap::Parser;

fn run_git_fetch() {
    let output = run_command(Command::new("git").arg("fetch"));
    if !output.status.success() {
        println!("Error fetching repository");
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
        println!("[WARN] Branch {} does not exist", branch);
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
    println!("[DEBUG] Running: \"{} {}\"", program, args);
    return command
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
}

fn run_git_status(folder_to_check: String, source_branch: String, target_branch: String) {
    match env::set_current_dir(folder_to_check.to_string()) {
        Ok(_) => {}
        _ => eprintln!("[ERROR] Going to directory {}", folder_to_check),
    }
    println!("[INFO] Checking folder {}", folder_to_check);
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
            println!(
                "[INFO] There are differences. You need to make a merge on {}",
                folder_to_check.to_string()
            );
        } else {
            println!("[INFO] There are no differences.");
        }
    } else {
        let err = String::from_utf8(output.stderr);
        let err_str = err.expect("Error getting error");
        if err_str.contains("not a git repository") {
            println!(
                "[WARN] folder {} is not a git repository",
                folder_to_check.to_string()
            );
        } else if err_str.contains("unknown revision or path not in the working tree") {
            eprintln!("[ERROR] Invalid branch for {}", folder_to_check);
        } else {
            eprintln!("[ERROR] Unknown error");
            eprintln!("err: {}", err_str);
        }
    }
}

fn get_projects_folders(path: String) -> Vec<String> {
    let projects_folder = match fs::read_dir(path.to_string()) {
        Ok(folder) => folder,
        _ => {
            eprintln!("[ERROR] Reading directory {}", path.to_string());
            exit(-1);
        }
    };

    let list_response: Vec<String> = projects_folder
        .into_iter()
        .map(|pf| pf.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    return list_response;
}

#[derive(Parser)]
struct Cli {
    #[arg(value_name = "projects-folder", help = "folder with projects to check")]
    projects_folder: String,

    #[arg(
        short,
        long,
        value_name = "source branch",
        help = "source branch to check (it must be a remote branch)"
    )]
    source_branch: String,

    #[arg(
        short,
        long,
        value_name = "target branch",
        default_value = "origin/main",
        help = "target branch to check (it must be a remote branch)"
    )]
    target_branch: String,

    #[arg(long)]
    debug: bool,
}

fn main() {
    let cli: Cli = Cli::parse();

    let projects_dir = cli.projects_folder;
    let source_branch = cli.source_branch;
    let target_branch = cli.target_branch;

    let projects_folders = get_projects_folders(projects_dir.to_string());
    projects_folders
        .into_iter()
        .map(|pf| format!("{}/{}", projects_dir.to_string(), pf))
        .filter(|pf| match metadata(pf) {
            Ok(md) => md.is_dir(),
            _ => false,
        })
        .for_each(|pf| run_git_status(pf, source_branch.to_string(), target_branch.to_string()));
}
