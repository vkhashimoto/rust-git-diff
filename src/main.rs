use std::{
    env, fs,
    process::{Command, Stdio},
};

fn run_git_fetch() {
    Command::new("git")
        .arg("fetch")
        .output()
        .expect("Error running git fetch");
}

fn run_git_status(folder_to_check: String, source_branch: String, target_branch: String) {
    env::set_current_dir(folder_to_check.to_string());
    run_git_fetch();
    let child = Command::new("git")
        .arg("diff")
        .arg(source_branch)
        .arg(target_branch)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    let output = child
        .expect("error running git status")
        .wait_with_output()
        .expect("error getting output");

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout).expect("error getting output");
        if !raw_output.is_empty() {
            println!(
                "There are differences. You need to make a merge on {}",
                folder_to_check.to_string()
            );
        } else {
            println!("There are no differences.");
        }
    } else {
        let err = String::from_utf8(output.stderr);
        let err_str = err.expect("Error getting error");
        if err_str.contains("not a git repository") {
            println!(
                "[WARN] folder {} is not a git repository",
                folder_to_check.to_string()
            );
        } else {
            println!("ERROR");
            println!("err: {}", err_str);
        }
    }
}

fn get_projects_folders(path: String) -> Vec<String> {
    let projects_folder = fs::read_dir(path).unwrap();

    let list_response: Vec<String> = projects_folder
        .into_iter()
        .map(|pf| pf.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    return list_response;
}
fn main() {
    let projects_dir = env::args().nth(1).unwrap_or(get_current_dir());
    let source_branch = env::args().nth(2).unwrap_or("origin/main".to_string());
    let target_branch = env::args().nth(3).unwrap_or("main".to_string());

    let projects_folders = get_projects_folders(projects_dir.to_string());
    projects_folders.into_iter().for_each(|pf| {
        run_git_status(
            format!("{}/{}", projects_dir.to_string(), pf),
            source_branch.to_string(),
            target_branch.to_string(),
        )
    });
}

fn get_current_dir() -> String {
    env::current_dir()
        .expect("error getting current dir")
        .as_os_str()
        .to_str()
        .expect("error getting current dir str")
        .to_string()
}
