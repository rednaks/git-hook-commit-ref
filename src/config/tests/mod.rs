use super::super::config;
use super::Config;
use std::collections::HashMap;

use std::env;
fn get_config_test_files_path() -> String {
    let path = env::current_dir().unwrap();
    format!("{}/src/config/tests/files/config", path.display())
}

fn use_git_config_file(git_config_file: String) {
    env::set_var(
        "GIT_CONFIG",
        format!("{}/{}", get_config_test_files_path(), git_config_file),
    );
}

#[test]
fn test_get_config_empty() {
    use_git_config_file(String::from("test_get_config_empty"));

    let config_map = config::get_config("commit-hook-ref".to_string()).unwrap();
    assert!(config_map.is_empty());
}

#[test]
fn test_get_config_without_org() {
    use_git_config_file(String::from("test_get_config_without_org"));

    let config_map = config::get_config("commit-hook-ref".to_string()).unwrap();

    assert!(config_map.get("org") == None);
}

#[test]
fn test_get_config_without_branches() {
    use_git_config_file(String::from("test_get_config_without_branches"));

    let config_map = config::get_config("commit-hook-ref".to_string()).unwrap();

    let config = config::Config::from_map(config_map).unwrap();

    let expected_branches = vec![String::from("master"), String::from("main")];
    assert_eq!(config.forbidden_branches, expected_branches)
}

#[test]
fn test_get_config_with_all_fields() {
    use_git_config_file(String::from("test_get_config_with_all_fields"));

    let config_map = config::get_config("commit-hook-ref".to_string()).unwrap();
    let config = match Config::from_map(config_map) {
        Ok(config) => config,
        Err(e) => {
            panic!("{}", e)
        }
    };

    assert_eq!(config.org, Some(String::from("test-org")));
    assert_eq!(config.project, "test-project");
    assert_eq!(config.forbidden_branches, vec!["master", "main", "prod"]);
}

#[test]
fn test_make_config_from_map() {
    let mut config_map: HashMap<String, String> = HashMap::new();

    config_map.insert(String::from("org"), String::from("test-org"));
    config_map.insert(String::from("project"), String::from("test-project"));
    config_map.insert(String::from("forbiddenbranches"), String::from("master"));

    let config = match Config::from_map(config_map) {
        Ok(config) => config,
        Err(e) => {
            panic!("{}", e)
        }
    };

    assert_eq!(config.org, Some(String::from("test-org")));
    assert_eq!(config.project, "test-project");
    assert_eq!(config.forbidden_branches, vec!["master"]);
}
