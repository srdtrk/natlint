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
        let violations: Vec<_> =
            lint(&content, &config.rule_set()).expect("Failed to process file");

        assert_eq!(violations.len(), 18);

        // Contract violations
        assert!(violations[0].0.rule_name == "MissingAuthor");
        assert!(violations[0].1 == 6);
        assert!(violations[0].0.rule_description == "Contracts must have an author comment");
        
        assert!(violations[1].0.rule_name == "MissingNotice");
        assert!(violations[1].1 == 6);
        assert!(violations[1].0.rule_description == "Contracts must have a notice comment");
        
        assert!(violations[2].0.rule_name == "MissingTitle");
        assert!(violations[2].1 == 6);
        assert!(violations[2].0.rule_description == "Contracts must have a title comment");

        // Variable violations
        assert!(violations[3].0.rule_name == "MissingInheritdoc");
        assert!(violations[3].1 == 7);
        assert!(violations[3].0.rule_description == "Public and override variables must have an inheritdoc comment.");
        
        assert!(violations[4].0.rule_name == "MissingNotice");
        assert!(violations[4].1 == 7);
        assert!(violations[4].0.rule_description == "Variables must have a notice or an inheritdoc comment.");

        // Error violations
        assert!(violations[5].0.rule_name == "MissingNotice");
        assert!(violations[5].1 == 11);
        assert!(violations[5].0.rule_description == "Errors must have a notice comment.");
        
        assert!(violations[6].0.rule_name == "MissingParam");
        assert!(violations[6].1 == 11);
        assert!(violations[6].0.rule_description == "Errors must document all parameters.");

        // Enum violations
        assert!(violations[7].0.rule_name == "MissingAuthor");
        assert!(violations[7].1 == 13);
        assert!(violations[7].0.rule_description == "Enums must have an author comment.");
        
        assert!(violations[8].0.rule_name == "MissingNotice");
        assert!(violations[8].1 == 13);
        assert!(violations[8].0.rule_description == "Enums must have a notice comment.");
        
        assert!(violations[9].0.rule_name == "MissingTitle");
        assert!(violations[9].1 == 13);
        assert!(violations[9].0.rule_description == "Enums must have a title comment.");
        
        assert!(violations[10].0.rule_name == "MissingVariant");
        assert!(violations[10].1 == 13);
        assert!(violations[10].0.rule_description == "Enums must document all variants.");

        // Struct violations
        assert!(violations[11].0.rule_name == "MissingAuthor");
        assert!(violations[11].1 == 15);
        assert!(violations[11].0.rule_description == "Structs must have an author comment.");
        
        assert!(violations[12].0.rule_name == "MissingNotice");
        assert!(violations[12].1 == 15);
        assert!(violations[12].0.rule_description == "Structs must have a notice comment.");
        
        assert!(violations[13].0.rule_name == "MissingParams");
        assert!(violations[13].1 == 15);
        assert!(violations[13].0.rule_description == "Structs must document all parameters.");
        
        assert!(violations[14].0.rule_name == "MissingTitle");
        assert!(violations[14].1 == 15);
        assert!(violations[14].0.rule_description == "Structs must have a title comment.");

        // Function violations
        assert!(violations[15].0.rule_name == "MissingInheritdoc");
        assert!(violations[15].1 == 22);
        assert!(violations[15].0.rule_description == "Public and override functions must have an inheritdoc comment.");
        
        assert!(violations[16].0.rule_name == "MissingNotice");
        assert!(violations[16].1 == 22);
        assert!(violations[16].0.rule_description == "Functions must have a notice or an inheritdoc comment.");
        
        assert!(violations[17].0.rule_name == "MissingParams");
        assert!(violations[17].1 == 22);
        assert!(violations[17].0.rule_description == "Functions must have their parameters documented or have an inheritdoc comment.");
    }

    #[test]
    fn test_interface() {
        let file_path = Path::new("tests/contracts/TestInterface.sol");
        let content = fs::read_to_string(file_path).expect("Failed to read test file");

        let config = load_default_config();
        let violations: Vec<_> =
            lint(&content, &config.rule_set()).expect("Failed to process file");

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
        let violations: Vec<_> =
            lint(&content, &config.rule_set()).expect("Failed to process file");

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
