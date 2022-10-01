use crate::check;
use crate::config;

impl Clone for check::Config {
    fn clone(&self) -> Self {
        config::Config {
            org: self.org.clone(),
            project: self.project.clone(),
            forbidden_branches: self.forbidden_branches.clone(),
        }
    }
}

#[test]
fn test_check_commit_msg_forbidden_branch() {
    let config = config::Config {
        org: Some("test-org".to_string()),
        project: "test-project".to_string(),
        forbidden_branches: vec!["master".to_string()],
    };

    let commit_msg = String::from("a commit msg");
    let result = check::check_commit_msg(config, &commit_msg, String::from("master"));

    assert_eq!(
        result,
        Err(String::from(
            "Branch name is forbidden, you can't commit here"
        ))
    );
}

#[test]
fn test_check_commit_msg_missing_ref() {
    let config = config::Config {
        org: Some("test-org".to_string()),
        project: "test-project".to_string(),
        forbidden_branches: vec!["master".to_string()],
    };

    let commit_msg = String::from("a commit msg");

    let result = check::check_commit_msg(
        config.clone(),
        &commit_msg,
        String::from(format!("{}-123", config.org.clone().unwrap())),
    );

    let expected_commit_msg = String::from(format!(
        "a commit msg - {}/{}#{}",
        config.org.clone().unwrap(),
        config.project,
        String::from("123")
    ));

    assert_eq!(result, Ok(expected_commit_msg));
}
#[test]
fn test_check_good_commit_msg() {
    let config = config::Config {
        org: Some(String::from("test-org")),
        project: String::from("test-project"),
        forbidden_branches: vec!["master".to_string()],
    };

    let commit_msg = String::from(format!(
        "a commit msg - {}/{}#{}",
        config.org.clone().unwrap(),
        config.project,
        String::from("123")
    ));

    let result = check::check_commit_msg(
        config.clone(),
        &commit_msg,
        String::from(format!("{}-123", config.org.clone().unwrap())),
    );

    assert_eq!(result, Ok(commit_msg));
}

#[test]
fn test_make_ref_org_not_in_branch_name() {
    let config = config::Config {
        org: Some(String::from("test-org")),
        project: String::from("test-project"),
        forbidden_branches: vec![String::from("master")],
    };

    let res = check::make_ref(config, String::from("other-org-123"));

    assert_eq!(
        res,
        Err(String::from(
            "Wrong branch name, missing organization should be formatted <org>-<issue_number>",
        ))
    );
}

#[test]
fn test_make_ref_branch_name_not_good_format() {
    let config = config::Config {
        org: Some(String::from("test-org")),
        project: String::from("test-project"),
        forbidden_branches: vec![String::from("master")],
    };

    let res = check::make_ref(
        config.clone(),
        String::from(format!("{}-short-description", config.org.unwrap())),
    );

    assert_eq!(
        res,
        Err(String::from(
            "Wrong branch name, should be formatted <org>-<issue_number>",
        ))
    );
}

#[test]
fn test_make_ref_without_org() {
    let config = config::Config {
        org: None,
        project: String::from("test-project"),
        forbidden_branches: vec![String::from("master")],
    };

    let res = check::make_ref(config.clone(), String::from("org-123"));

    assert_eq!(res, Ok(String::from(format!("{}#123", config.project))));
}
