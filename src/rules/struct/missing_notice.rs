use solang_parser::pt::StructDefinition;

crate::missing_comment_rule!(
    MissingNotice,
    StructDefinition,
    Notice,
    "Structs must have a notice comment."
);

#[cfg(test)]
mod tests {
    use super::{MissingNotice, StructDefinition};
    use crate::{
        generate_missing_comment_test_cases,
        parser::{CommentTag, CommentsRef, Parser},
        rules::{violation_error::ViolationError, Rule, Violation},
    };
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    macro_rules! test_missingnotice {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let item = child.as_struct().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);

                assert_eq!(MissingNotice::check(Some(parent), item, &comments), expected);
            }
        };
    }

    generate_missing_comment_test_cases!(
        Notice,
        test_missingnotice,
        MissingNotice,
        r"
            struct TestStruct {
                uint256 a;
            }
        ",
        "@notice",
        StructDefinition
    );

    test_missingnotice!(
        no_tag_no_violation,
        r"
        contract Test {
            /// Some notice
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None // WARNING: solang parser and the natspec docs interpret no tags as a notice
    );

    test_missingnotice!(
        multiline_no_tag_no_violation,
        r"
        contract Test {
            /**
             * Some comment
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None // WARNING: solang parser and the natspec docs interpret no tags as a notice
    );
}
