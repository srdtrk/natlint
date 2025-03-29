//! Test suite for natlint

mod natspec_test {
    use natlint::config::load_default_config;
    use natlint::linter::lint;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_contract() {
        let file_path = Path::new("tests/contracts/TestContract.sol");
        let content = fs::read_to_string(file_path).expect("Failed to read test file");

        let config = load_default_config();
        let violations: Vec<_> = lint(&content, &config).expect("Failed to process file");

        assert_eq!(violations.len(), 6);

        assert!(violations[0].0.rule_name == "MissingAuthor");
        assert!(violations[0].1 == 6);
        assert!(violations[1].0.rule_name == "MissingNotice");
        assert!(violations[1].1 == 6);
        assert!(violations[2].0.rule_name == "MissingTitle");
        assert!(violations[2].1 == 6);
        assert!(violations[3].0.rule_name == "MissingInheritdoc");
        assert!(violations[3].1 == 9);
        assert!(violations[4].0.rule_name == "MissingNotice");
        assert!(violations[4].1 == 9);
        assert!(violations[5].0.rule_name == "MissingParams");
        assert!(violations[5].1 == 9);
    }

    #[test]
    fn test_interface() {
        let file_path = Path::new("tests/contracts/TestInterface.sol");
        let content = fs::read_to_string(file_path).expect("Failed to read test file");

        let config = load_default_config();
        let violations: Vec<_> = lint(&content, &config).expect("Failed to process file");

        assert_eq!(violations.len(), 5);

        assert!(violations[0].0.rule_name == "MissingAuthor");
        assert!(violations[0].1 == 4);
        assert!(violations[1].0.rule_name == "MissingNotice");
        assert!(violations[1].1 == 4);
        assert!(violations[2].0.rule_name == "MissingTitle");
        assert!(violations[2].1 == 4);
        assert!(violations[3].0.rule_name == "MissingNotice");
        assert!(violations[3].1 == 5);
        assert!(violations[4].0.rule_name == "MissingParams");
        assert!(violations[4].1 == 5);
    }

    #[test]
    fn test_msgs() {
        let file_path = Path::new("tests/contracts/TestMsgs.sol");
        let content = fs::read_to_string(file_path).expect("Failed to read test file");

        let config = load_default_config();
        let violations: Vec<_> = lint(&content, &config).expect("Failed to process file");

        assert_eq!(violations.len(), 7);

        assert!(violations[0].0.rule_name == "MissingAuthor");
        assert!(violations[0].1 == 4);
        assert!(violations[1].0.rule_name == "MissingNotice");
        assert!(violations[1].1 == 4);
        assert!(violations[2].0.rule_name == "MissingTitle");
        assert!(violations[2].1 == 4);
        assert!(violations[3].0.rule_name == "MissingAuthor");
        assert!(violations[3].1 == 5);
        assert!(violations[4].0.rule_name == "MissingNotice");
        assert!(violations[4].1 == 5);
        assert!(violations[5].0.rule_name == "MissingParams");
        assert!(violations[5].1 == 5);
        assert!(violations[6].0.rule_name == "MissingTitle");
        assert!(violations[6].1 == 5);
    }
}
