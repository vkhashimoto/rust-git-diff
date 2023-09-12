use rust_git_diff::command::{CommandRunner, DefaultCommandRunner};
use rust_git_diff::git::GitCommand;
#[test]
fn git_repo_without_diff() {
//fn this_test() {
    let git_command = GitCommand::new("/tmp/git-without-diff").diff("origin", "featureA", "main");

    let executor = DefaultCommandRunner {};
    let command_result = executor.run(&git_command);

    assert!(
        command_result.0,
        "The status was: {} | {}",
        command_result.0,
        match command_result.2 {
            None => "There is no error output".to_string(),
            Some(output) => format!("The error output was: {}", output).to_string(),
        }
    );

    match command_result.1 {
        None => panic!("The output isn't present"),
        Some(output) => assert!(output.is_empty(), "The output wasn't empty: {}", output),
    };
}

#[test]
fn git_repo_with_diff() {
    let git_command = GitCommand::new("/tmp/git-with-diff").diff("origin", "featureA", "main");

    let executor = DefaultCommandRunner {};
    let command_result = executor.run(&git_command);

    assert!(
        command_result.0,
        "The status was: {} | {}",
        command_result.0,
        match command_result.2 {
            None => "There is no error output".to_string(),
            Some(output) => format!("The error output was: {}", output).to_string(),
        }
    );

    match command_result.1 {
        None => panic!("The output isn't present"),
        Some(output) => assert!(!output.is_empty(), "The output was empty: {}", output),
    };
}

#[test]
fn git_repo_with_diff_on_merge() {
    let git_command =
        GitCommand::new("/tmp/git-with-diff-on-merge").diff("upstream", "featureB", "main");

    let executor = DefaultCommandRunner {};
    let command_result = executor.run(&git_command);

    assert!(
        command_result.0,
        "The status was: {} | {}",
        command_result.0,
        match command_result.2 {
            None => "There is no error output".to_string(),
            Some(output) => format!("The error output was: {}", output).to_string(),
        }
    );

    match command_result.1 {
        None => panic!("The output isn't present"),
        Some(output) => assert!(!output.is_empty(), "The output was empty: {}", output),
    };
}

#[test]
fn git_repo_wrong_branch() {
    let git_command =
        GitCommand::new("/tmp/git-without-diff").diff("origin", "featureZ", "main");

    let executor = DefaultCommandRunner {};
    let command_result = executor.run(&git_command);

    assert!(
        command_result.0 == false,
        "The status was: {} | {}",
        command_result.0,
        match command_result.1 {
            None => "There is no success output".to_string(),
            Some(output) => format!("The success output was: {}", output).to_string(),
        }
    );

    match command_result.2 {
        None => panic!("The error output isn't present"),
        Some(output) => assert!(!output.is_empty(), "The error output was empty: {}", output),
    };

}

#[test]
fn git_repo_in_non_existing_folder() {
    let git_command =
        GitCommand::new("/tmp/non-existing-folder").diff("origin", "featureZ", "main");

    let executor = DefaultCommandRunner {};
    let command_result = executor.run(&git_command);

    assert!(
        command_result.0 == false,
        "The status was: {} | {}",
        command_result.0,
        match command_result.1 {
            None => "There is no success output".to_string(),
            Some(output) => format!("The success output was: {}", output).to_string(),
        }
    );

    match command_result.2 {
        None => panic!("The error output isn't present"),
        Some(output) => assert!(!output.is_empty(), "The error output was empty: {}", output),
    };

}
