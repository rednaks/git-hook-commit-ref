use std::env;
mod check;
mod cli;
mod config;
mod hook;

use git2::Repository;

fn main() -> Result<(), String> {
    let arg = match env::args().nth(1) {
        Some(val) => val,
        None => {
            return Err(String::from(
                "Missing argument. Usage: --install org git-commit-file-path",
            ))
        }
    };

    let current_dir = match std::env::current_dir() {
        Ok(cd) => cd,
        Err(_) => return Err(String::from("Unable to get current directory")),
    };

    let repo = match Repository::discover(current_dir) {
        Ok(r) => r,
        Err(_e) => {
            println!("Can't execute in a non git repository");
            return Err(String::from("Not a git repo"));
        }
    };

    if arg == "--install" {
        return cli::install(repo);
    } else if arg == "--check" {
        let config = match repo.config() {
            Ok(c) => c,
            Err(_) => return Err(String::from("Unable to get config")),
        };
        return cli::check(config);
    } else {
        return hook::handle_hook(arg, repo);
    }
}
