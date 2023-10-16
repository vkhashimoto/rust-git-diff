use std::{path::Path, process::Stdio};

use log::debug;

pub trait Command {
    fn get_command(&self) -> String;
    fn get_args(&self) -> Vec<String>;
    fn get_path(&self) -> &str;
}

pub type CommandResult = (bool, Option<String>, Option<String>);

pub trait CommandRunner {
    fn run(&self, command: &impl Command) -> CommandResult;
}

pub struct DefaultCommandRunner {}

impl CommandRunner for DefaultCommandRunner {
    fn run(&self, command: &impl Command) -> CommandResult {
        let str_command = command.get_command();
        let str_args = command.get_args();
        let path = command.get_path();
        let formatted_command_message = format!(
            "Command: '{} {}' in '{}'",
            str_command,
            str_args.join(" "),
            path
        );

        debug!("Running {}", formatted_command_message);

        debug!("Checking if path {} is a folder", path);
        if !Path::new(path).is_dir() {
            debug!("Not a folder");
            return (false, None, Some(format!("path {} is not a folder", path)));
        }

        debug!("Changing directory to {}", path);
        match std::env::set_current_dir(path) {
            Ok(_) => {}
            _ => {
                debug!("Directory not found");
                return (false, None, Some(format!("Directory not found {}", path)));
            }
        }
        let mut c = std::process::Command::new(str_command);

        let command_output = c
            .args(str_args)
            //.arg("fetch")
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect(&format!("Error running {}", formatted_command_message))
            .wait_with_output()
            .expect(&format!(
                "Error getting output for {}",
                formatted_command_message
            ));

        if command_output.status.success() {
            return (
                true,
                Some(String::from_utf8(command_output.stdout).expect(&format!(
                    "Error getting output from stdout for {}",
                    formatted_command_message
                ))),
                None,
            );
        } else {
            return (
                false,
                None,
                Some(String::from_utf8(command_output.stderr).expect(&format!(
                    "Error getting output from stderr for {}",
                    formatted_command_message
                ))),
            );
        }
    }
}
