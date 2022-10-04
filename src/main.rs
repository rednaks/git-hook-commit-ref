use std::env;
mod config;
use config::{get_config, Config};

mod check;
use check::{check_commit_msg, get_commit_msg, get_current_branch};

fn install() {
    println!("Installing ...");

    // 1. get current git root  .git
    let git_dir_path = match std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
    {
        Ok(output) => output,
        Err(_) => {
            println!("Not a git directory");
            std::process::exit(-1);
        }
    };

    let git_dir_path = match String::from_utf8(git_dir_path.stdout) {
        Ok(path) => path,
        Err(_) => {
            println!("Unable to decode stdout");
            std::process::exit(-1);
        }
    };

    // 2. get binary location
    let hook_bin_path = match env::args().nth(0) {
        Some(arg) => arg,
        None => {
            std::process::exit(-1);
        }
    };

    // 3. create a sym link to .git/hooks/prepare-commit-msg
    match std::process::Command::new("ln")
        .arg("-s")
        .arg(hook_bin_path)
        .arg(String::from(format!(
            "{}/.git/hooks/prepare-commit-msg",
            git_dir_path.trim()
        )))
        .status()
    {
        Ok(_) => {
            println!("Successfully created a symlink");
        }
        Err(_) => {
            println!("Unable to create symlink");
            std::process::exit(-1);
        }
    }
}

fn main() -> Result<(), String> {
    let arg = match env::args().nth(1) {
        Some(val) => val,
        None => {
            return Err(String::from(
                "Missing argument. Usage: --install org git-commit-file-path",
            ))
        }
    };

    if arg == "--install" {
        install();
        std::process::exit(0);
    }

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

    let commit_msg = get_commit_msg(&arg);

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
                match std::fs::write(arg, &new_commit_msg) {
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
