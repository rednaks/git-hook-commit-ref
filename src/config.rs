use git2;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Config {
    pub org: Option<String>,
    pub project: String,
    pub forbidden_branches: Vec<String>,
    pub branch_pattern: String,
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

        let branch_pattern = match config.get("branchpattern") {
            Some(pattern) => String::from(pattern),
            None => String::from("(?P<org>\\w+)-(?P<issue_number>\\d+)"),
        };

        Ok(Config {
            org,
            project,
            forbidden_branches,
            branch_pattern,
        })
    }
}

pub fn get_config(
    git_config: git2::Config,
    prefix: String,
) -> Result<HashMap<String, String>, String> {
    let mut config_entries = match git_config.entries(Some(format!("{}.*", prefix).as_str())) {
        Ok(entries) =>  entries,
        Err(_) => return Err(String::from("No entries found in config for {}. please make sure you configured correctly your repository"))
    };

    let mut config = HashMap::<String, String>::new();

    while let Some(entry) = config_entries.next() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => return Err(String::from("Error with config entry")),
        };

        let key = match entry.name() {
            Some(k) => k.replace(format!("{}.", prefix).as_str(), ""),
            None => return Err(String::from("Error parsing the config entry")),
        };
        let val = match entry.value() {
            Some(v) => String::from(v),
            None => return Err(format!("Unable to get value for config {}", key)),
        };
        config.insert(String::from(key), val);
    }

    Ok(config)
}
