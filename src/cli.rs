use crate::config;
use git2;
use std::env;

fn get_current_git_path(repo: git2::Repository) -> Result<String, String> {
    // get current git root  .git

    match repo.path().to_str() {
        Some(p) => Ok(String::from(p)),
        None => return Err(String::from("Unable to get repository path")),
    }
}

pub fn install(repo: git2::Repository) -> Result<(), String> {
    println!("Installing ...");

    // 2. get binary location
    let hook_bin_path = match env::args().nth(0) {
        Some(arg) => arg,
        None => {
            // should never happen
            std::process::exit(-1);
        }
    };

    let git_dir_path = match get_current_git_path(repo) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    // 3. create a sym link to .git/hooks/prepare-commit-msg
    match std::process::Command::new("ln")
        .arg("-s")
        .arg(hook_bin_path)
        .arg(String::from(format!(
            "{}hooks/prepare-commit-msg",
            git_dir_path.trim()
        )))
        .status()
    {
        Ok(_) => {
            println!("Successfully created a symlink");
            println!("You can now create the configuration file and use git hook-commit-ref --check to check if it's valid");
        }
        Err(_) => {
            return Err(String::from("Unable to create symlink"));
        }
    }

    Ok(())
}

pub fn check(config: git2::Config) -> Result<(), String> {
    let config_map = match config::get_config(config, String::from("commit-ref-hook")) {
        Ok(config_map) => config_map,
        Err(e) => return Err(e),
    };

    match config::Config::from_map(config_map) {
        Ok(_) => {
            println!("Everything looks good !")
        }
        Err(e) => return Err(e),
    }
    Ok(())
}
