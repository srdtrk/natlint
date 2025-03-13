//! Test suite for natlint

mod natspec_test {
    use std::path::Path;
    use std::collections::HashMap;
    use std::fs;
    
    use forge_fmt::Visitable;
    use natlint::{
        config::load_default_config,
        parser::{Comments, Parser}, 
        rules::Violation,
    };
    use solang_parser::parse;
    use std::any::Any;
    use line_col::LineColLookup;
    
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
                },
                natlint::parser::ParseSource::Function(function) => {
                    let violations = config.check_item(parent, function as &dyn Any, comments);
                    all_violations.extend(violations);
                },
                natlint::parser::ParseSource::Struct(structure) => {
                    let violations = config.check_item(parent, structure as &dyn Any, comments);
                    all_violations.extend(violations);
                },
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
    
    fn group_violations_by_rule(violations: &[Violation]) -> HashMap<String, Vec<&Violation>> {
        let mut grouped = HashMap::new();
        
        for violation in violations {
            grouped
                .entry(violation.rule.to_string())
                .or_insert_with(Vec::new)
                .push(violation);
        }
        
        grouped
    }
    
    #[test]
    fn test_iics26_router_msgs() {
        let file_path = Path::new("tests/contracts/IICS26RouterMsgs.sol");
        let content = fs::read_to_string(file_path).expect("Failed to read file");
        let line_lookup = LineColLookup::new(&content);
        let violations = check_file(file_path);
        let grouped = group_violations_by_rule(&violations);
        
        let violations_with_lines: Vec<_> = violations.iter()
            .map(|v| {
                let (line, _col) = line_lookup.get(v.loc.start());
                (v, line)
            })
            .collect();
        
        assert!(!violations.is_empty(), "Expected at least one violation");
        
        assert!(
            grouped.contains_key("MissingTitle"), 
            "Expected MissingTitle violation"
        );
        
        assert!(
            grouped.contains_key("MissingAuthor"), 
            "Expected MissingAuthor violation"
        );
        
        let interface_violations = violations_with_lines.iter()
            .filter(|(v, line)| {
                (v.rule == "MissingTitle" || v.rule == "MissingAuthor") && *line == 6
            })
            .count();
            
        assert!(interface_violations >= 2, 
            "Expected at least 2 violations (MissingTitle, MissingAuthor) for the interface at line 6");
        
        let packet_struct_violations = violations_with_lines.iter()
            .filter(|(_, line)| *line == 13)
            .count();
        
        assert!(packet_struct_violations > 0, 
            "Should have violations for the Packet struct at line 13");
        
        let msg_send_packet_violations = violations_with_lines.iter()
            .filter(|(v, line)| 
                (v.rule == "MissingTitle" || v.rule == "MissingAuthor" || v.rule == "MissingNotice") && 
                *line >= 40 && *line <= 45
            )
            .count();
            
        assert!(msg_send_packet_violations > 0, 
            "Should have violations for message structs");
        
        for (violation, line) in &violations_with_lines {
            assert!(*line > 0 && *line < 100, 
                "Line number should be within the file range, got {} for violation: {}", 
                line, violation.description);
        }
    }
}
