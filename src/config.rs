use std::collections::HashMap;
use std::process::Command;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Config {
    pub org: Option<String>,
    pub project: String,
    pub forbidden_branches: Vec<String>,
}

impl Config {
    pub fn from_map(config: HashMap<String, String>) -> Result<Config, String> {
        let mut forbidden_branches: Vec<String> = Vec::new();
        let forbidden_branches_fallback = String::from("master, main");
        let forbidden_branches_str = config
            .get("forbiddenbranches")
            .unwrap_or(&forbidden_branches_fallback);

        for fb in forbidden_branches_str.trim().to_string().split(", ") {
            forbidden_branches.push(String::from(fb));
        }

        let project = match config.get("project") {
            Some(project) => project.to_string(),
            None => return Err(String::from("Missing project in the config")),
        };

        let org = match config.get("org") {
            Some(org) => Some(org.clone()),
            None => return Err(String::from("Mirring org in the config")),
        };

        Ok(Config {
            org,
            project,
            forbidden_branches,
        })
    }
}

pub fn get_config(prefix: String) -> Result<HashMap<String, String>, String> {
    let git_config = match String::from_utf8(
        Command::new("git")
            .arg("config")
            .arg("--list")
            .output()
            .expect("Unable to find git config")
            .stdout,
    ) {
        Ok(c) => c,
        Err(_) => return Err(String::from("Error while parsing config")),
    };

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

    Ok(config)
}
