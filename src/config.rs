use std::process::Command;

#[derive(Debug)]
pub struct Config {
    pub org: String,
    pub project: String,
}

pub fn get_config() -> Config {
    let org_output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("commit-ref-hook.org")
        .output()
        .expect("Not configured, missing 'org' key ");

    let project_output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("commit-ref-hook.project")
        .output()
        .expect("Not configured, missing 'org' key ");

    Config {
        org: String::from_utf8(org_output.stdout)
            .unwrap()
            .trim()
            .to_string(),
        project: String::from_utf8(project_output.stdout)
            .unwrap()
            .trim()
            .to_string(),
    }
}
