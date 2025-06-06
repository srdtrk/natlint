//! Config test suite
use natlint::config::Config;
use std::path::Path;

#[test]
fn test_parse_empty_config() {
    // Test parsing an empty config file
    let empty_path = Path::new("tests/data/empty_config.toml");
    let empty_config_result = Config::from_file(empty_path);
    assert!(
        empty_config_result.is_ok(),
        "Failed to parse empty config: {:?}",
        empty_config_result.err()
    );
    // Empty config should deserialize to default values
    let empty_config = empty_config_result.unwrap();
    assert!(
        !empty_config.contract_rules.missing_author,
        "Default rule missing_author should be false"
    );
    assert!(
        empty_config.function_rules.missing_params,
        "Default rule missing_params should be true"
    );
}

#[test]
fn test_parse_sparse_config() {
    // Test parsing a config file with a few items set (the rest would be default == on)
    let sparse_path = Path::new("tests/data/sparse_config.toml");
    let sparse_config = Config::from_file(sparse_path).unwrap();
    // Check overridden rules
    assert!(
        !sparse_config.contract_rules.missing_author,
        "Sparse config should override missing_author to false"
    );
    assert!(
        !sparse_config.contract_rules.no_inheritdoc,
        "Sparse config should override no_inheritdoc to false"
    );
    assert!(
        !sparse_config.function_rules.missing_params,
        "Sparse config should override missing_params to false"
    );
    // Check a default rule that wasn't overridden
    assert!(
        sparse_config.contract_rules.missing_notice,
        "Default rule missing_notice should be true in sparse config"
    );
}

#[test]
fn test_parse_full_config() {
    // Test parsing a full config file
    let full_path = Path::new("tests/data/full_config.toml");
    let full_config_result = Config::from_file(full_path);
    assert!(
        full_config_result.is_ok(),
        "Failed to parse full config: {:?}",
        full_config_result.err()
    );
    let full_config = full_config_result.unwrap();
    // Check a few rules to ensure they are all Some (since set to true)
    assert!(
        full_config.contract_rules.missing_author,
        "Full config rule missing_author should be true"
    );
    assert!(
        full_config.error_rules.missing_param,
        "Full config rule missing_param should be true"
    );
    assert!(
        full_config.variable_rules.no_title,
        "Full config rule no_title should be true"
    );
}
