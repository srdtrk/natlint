//! Lint test suite

use natlint::config::Config;
use natlint::linter::lint;
use std::fs;
use std::path::Path;

#[test]
fn test_contract() {
    let file_path = Path::new("tests/data/TestContract.sol");
    let content = fs::read_to_string(file_path).expect("Failed to read test file");

    let config = Config::default();
    let violations: Vec<_> = lint(&content, &config.rules()).expect("Failed to process file");

    assert_eq!(violations.len(), 14);

    assert_eq!(violations[0].0.rule_name, "MissingNotice");
    assert_eq!(violations[0].1, 6);
    assert_eq!(
        violations[0].0.rule_description,
        "Contracts must have a notice comment."
    );

    assert_eq!(violations[1].0.rule_name, "MissingTitle");
    assert_eq!(violations[1].1, 6);
    assert_eq!(
        violations[1].0.rule_description,
        "Contracts must have a title comment."
    );

    // Variable vioations
    assert_eq!(violations[2].0.rule_name, "MissingInheritdoc");
    assert_eq!(violations[2].1, 7);
    assert_eq!(
        violations[2].0.rule_description,
        "Public and override variables must have an inheritdoc comment."
    );

    assert_eq!(violations[3].0.rule_name, "MissingNotice");
    assert_eq!(violations[3].1, 7);
    assert_eq!(
        violations[3].0.rule_description,
        "Variables must have a notice or an inheritdoc comment."
    );

    // Event violations
    assert_eq!(violations[4].0.rule_name, "MissingNotice");
    assert_eq!(violations[4].1, 9);
    assert_eq!(
        violations[4].0.rule_description,
        "Events must have a notice comment."
    );

    assert_eq!(violations[5].0.rule_name, "MissingParam");
    assert_eq!(violations[5].1, 9);
    assert_eq!(
        violations[5].0.rule_description,
        "Events must document all parameters."
    );

    // Error violations
    assert_eq!(violations[6].0.rule_name, "MissingNotice");
    assert_eq!(violations[6].1, 11);
    assert_eq!(
        violations[6].0.rule_description,
        "Errors must have a notice comment."
    );

    assert_eq!(violations[7].0.rule_name, "MissingParam");
    assert_eq!(violations[7].1, 11);
    assert_eq!(
        violations[7].0.rule_description,
        "Errors must document all parameters."
    );

    // Enum violations
    assert_eq!(violations[8].0.rule_name, "MissingNotice");
    assert_eq!(violations[8].1, 13);
    assert_eq!(
        violations[8].0.rule_description,
        "Enums must have a notice comment."
    );

    // Struct violations
    assert_eq!(violations[9].0.rule_name, "MissingNotice");
    assert_eq!(violations[9].1, 15);
    assert_eq!(
        violations[9].0.rule_description,
        "Structs must have a notice comment."
    );

    assert_eq!(violations[10].0.rule_name, "MissingParams");
    assert_eq!(violations[10].1, 15);
    assert_eq!(
        violations[10].0.rule_description,
        "Structs must document all parameters."
    );

    // Function violations
    assert_eq!(violations[11].0.rule_name, "MissingInheritdoc");
    assert_eq!(violations[11].1, 22);
    assert_eq!(
        violations[11].0.rule_description,
        "Public and override functions must have an inheritdoc comment."
    );

    assert_eq!(violations[12].0.rule_name, "MissingNotice");
    assert_eq!(violations[12].1, 22);
    assert_eq!(
        violations[12].0.rule_description,
        "Functions must have a notice or an inheritdoc comment."
    );

    assert_eq!(violations[13].0.rule_name, "MissingParams");
    assert_eq!(violations[13].1, 22);
    assert_eq!(
        violations[13].0.rule_description,
        "Functions must have their parameters documented or have an inheritdoc comment."
    );
}

#[test]
fn test_interface() {
    let file_path = Path::new("tests/data/TestInterface.sol");
    let content = fs::read_to_string(file_path).expect("Failed to read test file");

    let config = Config::default();
    let violations: Vec<_> = lint(&content, &config.rules()).expect("Failed to process file");

    assert_eq!(violations.len(), 4);

    assert_eq!(violations[0].0.rule_name, "MissingNotice");
    assert_eq!(violations[0].1, 4);
    assert_eq!(violations[1].0.rule_name, "MissingTitle");
    assert_eq!(violations[1].1, 4);
    assert_eq!(violations[2].0.rule_name, "MissingNotice");
    assert_eq!(violations[2].1, 5);
    assert_eq!(violations[3].0.rule_name, "MissingParams");
    assert_eq!(violations[3].1, 5);
}

#[test]
fn test_msgs() {
    let file_path = Path::new("tests/data/TestMsgs.sol");
    let content = fs::read_to_string(file_path).expect("Failed to read test file");

    let config = Config::default();
    let violations: Vec<_> = lint(&content, &config.rules()).expect("Failed to process file");

    assert_eq!(violations.len(), 4);

    assert_eq!(violations[0].0.rule_name, "MissingNotice");
    assert_eq!(violations[0].1, 4);
    assert_eq!(violations[1].0.rule_name, "MissingTitle");
    assert_eq!(violations[1].1, 4);
    assert_eq!(violations[2].0.rule_name, "MissingNotice");
    assert_eq!(violations[2].1, 5);
    assert_eq!(violations[3].0.rule_name, "MissingParams");
    assert_eq!(violations[3].1, 5);
}

#[test]
fn test_contract_with_disable() {
    let file_path = Path::new("tests/data/TestContractWithDisable.sol");
    let content = fs::read_to_string(file_path).expect("Failed to read test file");

    let config = Config::default();
    let violations: Vec<_> = lint(&content, &config.rules()).expect("Failed to process file");

    assert_eq!(violations.len(), 8);

    assert_eq!(violations[0].0.rule_name, "MissingNotice");
    assert_eq!(violations[0].1, 6);
    assert_eq!(
        violations[0].0.rule_description,
        "Contracts must have a notice comment."
    );

    assert_eq!(violations[1].0.rule_name, "MissingTitle");
    assert_eq!(violations[1].1, 6);
    assert_eq!(
        violations[1].0.rule_description,
        "Contracts must have a title comment."
    );

    // Variable vioations
    assert_eq!(violations[2].0.rule_name, "MissingInheritdoc");
    assert_eq!(violations[2].1, 7);
    assert_eq!(
        violations[2].0.rule_description,
        "Public and override variables must have an inheritdoc comment."
    );

    assert_eq!(violations[3].0.rule_name, "MissingNotice");
    assert_eq!(violations[3].1, 7);
    assert_eq!(
        violations[3].0.rule_description,
        "Variables must have a notice or an inheritdoc comment."
    );

    // Event violations
    assert_eq!(violations[4].0.rule_name, "MissingNotice");
    assert_eq!(violations[4].1, 9);
    assert_eq!(
        violations[4].0.rule_description,
        "Events must have a notice comment."
    );

    assert_eq!(violations[5].0.rule_name, "MissingParam");
    assert_eq!(violations[5].1, 9);
    assert_eq!(
        violations[5].0.rule_description,
        "Events must document all parameters."
    );

    // No Error violations

    // Enum violations
    assert_eq!(violations[6].0.rule_name, "MissingNotice");
    assert_eq!(violations[6].1, 14);
    assert_eq!(
        violations[6].0.rule_description,
        "Enums must have a notice comment."
    );

    // Struct violations
    assert_eq!(violations[7].0.rule_name, "MissingNotice");
    assert_eq!(violations[7].1, 17);
    assert_eq!(
        violations[7].0.rule_description,
        "Structs must have a notice comment."
    );

    // No Function violations
}
