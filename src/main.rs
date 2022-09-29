use std::env;
use std::process::Command;
mod config;
use config::{get_config, Config};

fn get_current_branch() -> String {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("failed to execute git command");

    let stdout = String::from_utf8(output.stdout).unwrap();

    stdout
        .lines()
        .nth(0)
        .expect("Failed git execute git")
        .to_string()
        .trim()
        .to_string()
}

fn get_commit_msg(commit_msg_file: &String) -> String {
    std::fs::read_to_string(commit_msg_file).expect("Unable to read commit file msg")
}

fn make_ref(config: Config, branch: String) -> Result<String, String> {
    if !branch.contains(&config.org) {
        return Err(String::from(
            "Wrong branch name, should be formatted <org>-<issue_number>",
        ));
    }

    match branch.split('-').last().unwrap().parse::<u16>() {
        Ok(issue_number) => Ok(String::from(format!(
            "{}/{}#{}",
            config.org, config.project, issue_number
        ))),
        Err(_) => Err(String::from(
            "Wrong branch name, should be formatted <org>-<issue_number>",
        )),
    }
}

fn check_commit_msg(config: Config, commit_msg: &String, branch: String) -> Result<String, String> {
    // check if branch is good:

    if config.forbidden_branches.contains(&branch) {
        return Err(String::from(
            "Branch name is forbidden, you can't commit here",
        ));
    }

    match make_ref(config, branch) {
        Ok(branch_ref) => {
            if !commit_msg.contains(&branch_ref) {
                Ok(String::from(format!(
                    "{} - {}",
                    commit_msg.trim(),
                    branch_ref
                )))
            } else {
                Ok(String::from(commit_msg))
            }
        }
        Err(e) => Err(e),
    }
}

fn main() {
    let commit_msg_file = env::args().nth(1).expect("No file name provided");

    // get current branch
    // check if commit msg contains the reference
    // check if we're in the good branch, if the reference matches the branch name,
    // or the branch is not in the correct pattern: (<org>/<project_name>-<issue_number>)
    // if branch is missing, reconstruct it and add it to the msg: (<org>/<project_name>#<issue_number>)

    let current_branch = get_current_branch();

    let commit_msg = get_commit_msg(&commit_msg_file);

    let config = Config::from_map(get_config(String::from("commit-ref-hook")));

    match check_commit_msg(config, &commit_msg, current_branch) {
        Ok(new_commit_msg) => {
            if new_commit_msg != commit_msg {
                match std::fs::write(commit_msg_file, &new_commit_msg) {
                    Ok(_) => {
                        println!("Updated commit msg ! : {}", new_commit_msg);
                    }
                    Err(_) => {
                        println!("unable to update commit msg !");
                    }
                }
            } else {
                println!("Everthing is good.");
            }
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
    }
}
