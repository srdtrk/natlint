//! Test suite for natlint

mod natspec_test {
    use std::fs;
    use std::path::Path;

    use forge_fmt::Visitable;
    use line_col::LineColLookup;
    use natlint::{
        config::load_default_config,
        parser::{Comments, Parser},
        rules::Violation,
    };
    use solang_parser::parse;
    use std::any::Any;

    #[allow(clippy::needless_pass_by_value)]
    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    fn check_file(file_path: &Path) -> Vec<Violation> {
        let content = std::fs::read_to_string(file_path).expect("failed to read file");
        let parser = parse_source(&content);
        let config = load_default_config();
        let mut all_violations = Vec::new();

        fn process_item(
            item: &natlint::parser::ParseItem,
            parent: Option<&natlint::parser::ParseItem>,
            config: &natlint::config::Config,
            comments: &Comments,
            all_violations: &mut Vec<Violation>,
        ) {
            match &item.source {
                natlint::parser::ParseSource::Contract(contract) => {
                    let violations = config.check_item(parent, &**contract as &dyn Any, comments);
                    all_violations.extend(violations);
                }
                natlint::parser::ParseSource::Function(function) => {
                    let violations = config.check_item(parent, function as &dyn Any, comments);
                    all_violations.extend(violations);
                }
                natlint::parser::ParseSource::Struct(structure) => {
                    let violations = config.check_item(parent, structure as &dyn Any, comments);
                    all_violations.extend(violations);
                }
                _ => {}
            }

            for child in &item.children {
                process_item(child, Some(item), config, &item.comments, all_violations);
            }
        }

        for item in &parser.items() {
            process_item(item, None, &config, &item.comments, &mut all_violations);
        }

        all_violations
    }

    #[test]
    fn test_contract() {
        let file_path = Path::new("tests/contracts/TestContract.sol");
        let content = fs::read_to_string(file_path).expect("Failed to read file");
        let line_lookup = LineColLookup::new(&content);
        let violations = check_file(file_path);
        let violations_with_lines: Vec<_> = violations
            .iter()
            .map(|v| {
                let (line, _col) = line_lookup.get(v.loc.start());
                (v, line)
            })
            .collect();

        assert_eq!(violations.len(), 6);

        assert!(violations_with_lines[0].0.rule_name == "MissingAuthor");
        assert!(violations_with_lines[0].1 == 6);
        assert!(violations_with_lines[1].0.rule_name == "MissingNotice");
        assert!(violations_with_lines[1].1 == 6);
        assert!(violations_with_lines[2].0.rule_name == "MissingTitle");
        assert!(violations_with_lines[2].1 == 6);
        assert!(violations_with_lines[3].0.rule_name == "MissingInheritdoc");
        assert!(violations_with_lines[3].1 == 9);
        assert!(violations_with_lines[4].0.rule_name == "MissingNotice");
        assert!(violations_with_lines[4].1 == 9);
        assert!(violations_with_lines[5].0.rule_name == "MissingParams");
        assert!(violations_with_lines[5].1 == 9);
    }
}
