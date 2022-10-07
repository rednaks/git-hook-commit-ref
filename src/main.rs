use std::env;
mod check;
mod cli;
mod config;
mod hook;

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
        return cli::install();
    } else if arg == "--check" {
        return cli::check();
    } else {
        return hook::handle_hook(arg);
    }
}
