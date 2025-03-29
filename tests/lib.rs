//! Test suite for natlint

mod natspec_test {
    use natlint::config::load_default_config;
    use natlint::parser::process_file::process_file;
    use std::path::Path;

    #[test]
    fn test_contract() {
        let file_path = Path::new("tests/contracts/TestContract.sol");
        let config = load_default_config();
        let violations: Vec<_> = process_file(file_path, &config)
            .expect("Failed to process file")
            .into_iter()
            .map(|(_, violation, line)| (violation, line))
            .collect();

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
}
