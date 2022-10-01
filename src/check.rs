use super::config::Config;
use std::process::Command;

#[cfg(test)]
mod tests;

pub fn get_current_branch() -> Result<String, String> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("failed to execute git command");

    let stdout = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(_) => return Err(String::from("Output issue")),
    };

    Ok(stdout
        .lines()
        .nth(0)
        .expect("Failed git execute git")
        .to_string()
        .trim()
        .to_string())
}

pub fn get_commit_msg(commit_msg_file: &String) -> String {
    std::fs::read_to_string(commit_msg_file).expect("Unable to read commit file msg")
}

pub fn make_ref(config: Config, branch: String) -> Result<String, String> {
    match config.org.clone() {
        Some(org) => {
            if !branch.contains(&org) {
                return Err(String::from(
                    "Wrong branch name, missing organization should be formatted <org>-<issue_number>",
                ));
            }
        }
        None => {
            // nothing to do
        }
    }

    let issue_number_part = match branch.split('-').last() {
        Some(part) => part,
        None => {
            return Err(String::from(
                "Wrong branch name, should be formatted <org>-<issue_number>",
            ))
        }
    };

    match issue_number_part.parse::<u16>() {
        Ok(issue_number) => {
            let org = match config.org {
                Some(org) => org,
                None => {
                    return Ok(String::from(format!("{}#{}", config.project, issue_number)));
                }
            };

            Ok(String::from(format!(
                "{}/{}#{}",
                org, config.project, issue_number
            )))
        }

        Err(_) => Err(String::from(
            "Wrong branch name, should be formatted <org>-<issue_number>",
        )),
    }
}

pub fn check_commit_msg(
    config: Config,
    commit_msg: &String,
    branch: String,
) -> Result<String, String> {
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
