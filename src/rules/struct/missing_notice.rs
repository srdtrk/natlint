use solang_parser::pt::StructDefinition;

use crate::parser::{CommentTag, CommentsRef, ParseItem};

use super::super::{Rule, Violation};

/// This rule requires that all structs have a notice comment.
pub struct MissingNotice;

impl Rule<StructDefinition> for MissingNotice {
    const NAME: &'static str = "Missing Notice";
    const DESCRIPTION: &'static str = "This rule requires that all structs have a notice comment.";

    fn check(
        _: Option<&ParseItem>,
        item: &StructDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // Function must have a notice comment
        match comments.include_tag(CommentTag::Notice).len() {
            0 => Some(Violation::new(
                Self::NAME,
                "Missing a notice comment".to_string(),
                item.loc,
            )),
            1 => None,
            _ => Some(Violation::new(
                Self::NAME,
                "Too many notice comments".to_string(),
                item.loc,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MissingNotice, Rule, StructDefinition};
    use crate::{
        parser::{CommentsRef, Parser},
        rules::Violation,
    };
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    /// Macro to define a test case for `MissingParams` rule
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

                assert_eq!(MissingNotice::check(Some(parent), item, comments), expected);
            }
        };
    }

    test_missingnotice!(
        no_violation,
        r"
        interface Test {
            /// @notice Some notice
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_missingnotice!(
        multi_no_violation,
        r"
        interface Test {
            /// @notice Some notice
            /// @param a Some param
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_missingnotice!(
        multiline_no_violation,
        r"
        interface Test {
            /**
             * @notice Some notice
             * @param a Some param
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_missingnotice!(
        empty_violation,
        r"
        contract Test {
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            MissingNotice::NAME,
            "Missing a notice comment".to_string(),
            sct.loc
        ))
    );

    test_missingnotice!(
        no_tag_violation,
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
        violation,
        r"
        contract Test {
            /// @param a Some param
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            MissingNotice::NAME,
            "Missing a notice comment".to_string(),
            sct.loc
        ))
    );

    test_missingnotice!(
        multi_violation,
        r"
        contract Test {
            /// @notice a Some struct
            /// @notice b Some struct
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            MissingNotice::NAME,
            "Too many notice comments".to_string(),
            sct.loc
        ))
    );

    test_missingnotice!(
        multiline_violation,
        r"
        contract Test {
            /**
             * @param a Some param
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            MissingNotice::NAME,
            "Missing a notice comment".to_string(),
            sct.loc
        ))
    );

    test_missingnotice!(
        multiline_no_tag_violation,
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

    test_missingnotice!(
        multiline_multi_violation,
        r"
        contract Test {
            /**
             * @notice a Some struct
             * @notice b Some struct
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            MissingNotice::NAME,
            "Too many notice comments".to_string(),
            sct.loc
        ))
    );
}
