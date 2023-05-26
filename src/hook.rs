use crate::config::{get_config, Config};

use crate::check::{check_commit_msg, get_commit_msg, get_current_branch};
use git2;
use std::env;

pub fn handle_hook(commit_file_path: String, repo: git2::Repository) -> Result<(), String> {
    // add config check arg

    // get current branch
    // check if commit msg contains the reference
    // check if we're in the good branch, if the reference matches the branch name,
    // or the branch is not in the correct pattern: (<org>/<project_name>-<issue_number>)
    // if branch is missing, reconstruct it and add it to the msg: (<org>/<project_name>#<issue_number>)

    let ignore_hook: bool = match env::var("COMMIT_HOOK_IGNORE") {
        Ok(v) => v.parse::<bool>().unwrap_or(false),
        Err(_) => false,
    };

    if ignore_hook {
        println!("Bypassing hook because COMMIT_HOOK_IGNORE=true");
        return Ok(());
    }

    let current_branch = match get_current_branch(&repo) {
        Ok(branch) => branch,
        Err(e) => return Err(e),
    };

    let commit_msg = get_commit_msg(&commit_file_path);
    let git_config = match repo.config() {
        Ok(cfg) => cfg,
        Err(_) => return Err(String::from("No config found")),
    };

    let config_map = match get_config(git_config, String::from("commit-ref-hook")) {
        Ok(cm) => cm,
        Err(e) => return Err(e),
    };

    let config = match Config::from_map(config_map) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    match check_commit_msg(config, &commit_msg, current_branch) {
        Ok(new_commit_msg) => {
            if new_commit_msg != commit_msg {
                match std::fs::write(commit_file_path, &new_commit_msg) {
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
            println!("If you want to bypass the hook, use COMMIT_HOOK_IGNORE=true git commit ...");
            std::process::exit(-1);
        }
    }

    Ok(())
}
