use std::collections::HashMap;
use std::process::Command;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Config {
    pub org: String,
    pub project: String,
    pub forbidden_branches: Vec<String>,
}

impl Config {
    pub fn from_map(config: HashMap<String, String>) -> Config {
        let mut forbidden_branches: Vec<String> = Vec::new();
        for fb in config
            .get("forbiddenbranches")
            .unwrap()
            .trim()
            .to_string()
            .split(", ")
        {
            forbidden_branches.push(fb.to_string());
        }

        Config {
            org: String::from(config.get("org").unwrap()),
            project: String::from(config.get("project").unwrap()),
            forbidden_branches,
        }
    }
}

pub fn get_config(prefix: String) -> HashMap<String, String> {
    let git_config = String::from_utf8(
        Command::new("git")
            .arg("config")
            .arg("--list")
            .output()
            .expect("Unable to find git config")
            .stdout,
    )
    .unwrap();

    let hook_config = git_config
        .lines()
        .filter(|line| line.starts_with(prefix.as_str()));

    let mut config = HashMap::<String, String>::new();

    for c in hook_config {
        let kv_clean = c.replace(format!("{}.", prefix).as_str(), "");
        let mut kv = kv_clean.split('=');
        config.insert(
            String::from(kv.next().unwrap()),
            String::from(kv.next().unwrap()),
        );
    }

    config
}
