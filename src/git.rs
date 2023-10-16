use crate::command::Command;

pub struct Repo {
    pub name: String,
    pub path: String,
    pub remote_name: String,
    pub source_branch: String,
    pub target_branch: String,
}

impl Repo {
    pub fn new(
        name: &str,
        path: &str,
        remote_name: &str,
        source_branch: &str,
        target_branch: &str,
    ) -> Repo {
        Repo {
            name: String::from(name),
            path: String::from(path),
            remote_name: String::from(remote_name),
            source_branch: String::from(source_branch),
            target_branch: String::from(target_branch),
        }
    }

    pub fn fetch(&self) -> Vec<GitCommand> {
        let mut commands = Vec::new();

        commands.push(GitCommand::new(&self.path).fetch(&self.remote_name, &self.source_branch));
        commands.push(GitCommand::new(&self.path).fetch(&self.remote_name, &self.target_branch));

        commands
    }

    pub fn diff(&self) -> GitCommand {
        GitCommand::new(&self.path).diff(
            &self.remote_name,
            &self.source_branch,
            &self.target_branch,
        )
    }
}

pub struct Git {
    pub repositories: Vec<Repo>,
}

impl Git {
    pub fn new() -> Git {
        Git {
            repositories: Vec::new(),
        }
    }

    pub fn add_repository(&mut self, repository: Repo) {
        self.repositories.push(repository);
    }
}

#[derive(Clone)]
pub struct GitCommand {
    command: String,
    args: Vec<String>,
    path: String,
}

impl GitCommand {
    pub fn new(path: &str) -> GitCommand {
        let command = GitCommand {
            command: "git".to_owned(),
            args: Vec::new(),
            path: String::from(path),
        };
        command
    }

    pub fn diff(
        mut self,
        remote_name: &str,
        source_branch: &str,
        target_branch: &str,
    ) -> GitCommand {
        self.args.push(String::from("log"));
        self.args.push(String::from("--no-merges"));
        self.args.push(format!("{}/{}", remote_name, source_branch));
        self.args
            .push(format!("^{}/{}", remote_name, target_branch));
        self
    }

    pub fn fetch(mut self, remote_name: &str, branch: &str) -> GitCommand {
        self.args.push(String::from("fetch"));
        self.args.push(String::from(remote_name));
        self.args.push(String::from(branch));

        self
    }
}

impl Command for GitCommand {
    fn get_command(&self) -> String {
        self.command.clone()
    }

    fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    fn get_path(&self) -> &str {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::{Git, GitCommand, Repo};
    use crate::command::Command;

    #[test]
    fn add_repository() {
        let mut git =
            Git::new();
        git.add_repository(Repo::new("git-test", "/tmp/git-test", "origin", "develop", "main"));

        assert_eq!(true, git.repositories.get(0).is_some());
        assert_eq!(
            String::from("/tmp/git-test"),
            git.repositories.get(0).unwrap().path
        );
        assert_eq!(
            String::from("develop"),
            git.repositories.get(0).unwrap().source_branch
        );
        assert_eq!(
            String::from("main"),
            git.repositories.get(0).unwrap().target_branch
        );
    }

    #[test]
    fn git_command_diff() {
        let command = GitCommand::new("/tmp/git-test").diff("origin", "develop", "main");

        assert_eq!("git", command.get_command());
        assert_eq!(
            "log --no-merges origin/develop ^origin/main",
            command.get_args().join(" ")
        );
        assert_eq!("/tmp/git-test", command.get_path());
    }

    #[test]
    fn git_command_fetch() {
        let command = GitCommand::new("/tmp/git-test").fetch("origin", "main");

        assert_eq!("git", command.get_command());
        assert_eq!("fetch origin main", command.get_args().join(" "));
        assert_eq!("/tmp/git-test", command.get_path());
    }

    #[test]
    fn repo_fetch() {
        let repo = Repo::new("git-test", "/tmp/git-test", "origin", "develop", "main");

        let fetch = repo.fetch();
        let commands = fetch
            .iter()
            .map(|f| f.get_command())
            .collect::<Vec<String>>();
        let args = fetch
            .iter()
            .map(|f| f.get_args().join(" "))
            .collect::<Vec<String>>();

        assert_eq!(2, fetch.len());

        assert_eq!("git", commands.get(0).unwrap());
        assert_eq!("fetch origin develop", args.get(0).unwrap());

        assert_eq!("git", commands.get(1).unwrap());
        assert_eq!("fetch origin main", args.get(1).unwrap());
    }

    #[test]
    fn repo_diff() {
        let repo = Repo::new("git-test", "/tmp/git-test", "origin", "develop", "main");

        let diff = repo.diff();
        let command = diff.get_command();
        let args = diff.get_args().join(" ");

        assert_eq!("git", command);
        assert_eq!("log --no-merges origin/develop ^origin/main", args);
    }
}
