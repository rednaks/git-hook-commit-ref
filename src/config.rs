use std::process::Command;

#[derive(Debug)]
pub struct Config {
    pub org: String,
    pub project: String,
    pub forbidden_branches: Vec<String>,
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
        .expect("Not configured, missing 'project' key ");

    let forbidden_branches_output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("commit-ref-hook.forbiddenbranches")
        .output()
        .expect("Not configured, missing 'forbiddenbranches' key ")
        .stdout;

    let mut forbidden_branches: Vec<String> = Vec::new();

    for b in String::from_utf8(forbidden_branches_output)
        .unwrap()
        .trim()
        .to_string()
        .split(", ")
    {
        forbidden_branches.push(b.to_string());
    }

    Config {
        org: String::from_utf8(org_output.stdout)
            .unwrap()
            .trim()
            .to_string(),
        project: String::from_utf8(project_output.stdout)
            .unwrap()
            .trim()
            .to_string(),
        forbidden_branches,
    }
}
