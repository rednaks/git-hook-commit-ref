use crate::config::{get_config, Config};

use crate::check::{check_commit_msg, get_commit_msg, get_current_branch};

pub fn handle_hook(commit_file_path: String) -> Result<(), String> {
    // add config check arg

    // get current branch
    // check if commit msg contains the reference
    // check if we're in the good branch, if the reference matches the branch name,
    // or the branch is not in the correct pattern: (<org>/<project_name>-<issue_number>)
    // if branch is missing, reconstruct it and add it to the msg: (<org>/<project_name>#<issue_number>)

    let current_branch = match get_current_branch() {
        Ok(branch) => branch,
        Err(e) => return Err(e),
    };

    let commit_msg = get_commit_msg(&commit_file_path);

    let config_map = match get_config(String::from("commit-ref-hook")) {
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
            std::process::exit(-1);
        }
    }

    Ok(())
}
