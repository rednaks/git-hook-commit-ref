use std::env;
mod config;
use config::{get_config, Config};

mod check;
use check::{check_commit_msg, get_commit_msg, get_current_branch};

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
