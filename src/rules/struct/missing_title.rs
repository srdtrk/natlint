//! This rule requires that all structs have a title comment.
//! This rule will be off by default.

use solang_parser::pt::StructDefinition;

use crate::parser::{CommentTag, CommentsRef, ParseItem};

use super::super::{Rule, Violation};

/// This rule requires that all structs have a title comment.
pub struct MissingTitle;

impl Rule<StructDefinition> for MissingTitle {
    const NAME: &'static str = "Missing Title";
    const DESCRIPTION: &'static str = "This rule requires that all structs have a title comment.";

    fn check(
        _: Option<&ParseItem>,
        item: &StructDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // Function must have a title comment
        match comments.include_tag(CommentTag::Title).len() {
            0 => Some(Violation::new(
                Self::NAME,
                "Missing a title comment".to_string(),
                item.loc,
            )),
            1 => None,
            _ => Some(Violation::new(
                Self::NAME,
                "Too many title comments".to_string(),
                item.loc,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MissingTitle, Rule, StructDefinition};
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
    macro_rules! test_missingtitle {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let item = child.as_struct().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);

                assert_eq!(MissingTitle::check(Some(parent), item, comments), expected);
            }
        };
    }

    test_missingtitle!(
        no_violation,
        r"
        interface Test {
            /// @title Some title
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_missingtitle!(
        multi_no_violation,
        r"
        interface Test {
            /// @title Some title
            /// @param a Some param
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_missingtitle!(
        multiline_no_violation,
        r"
        interface Test {
            /**
             * @title Some title
             * @param a Some param
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_missingtitle!(
        empty_violation,
        r"
        contract Test {
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            MissingTitle::NAME,
            "Missing a title comment".to_string(),
            sct.loc
        ))
    );

    test_missingtitle!(
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
            MissingTitle::NAME,
            "Missing a title comment".to_string(),
            sct.loc
        ))
    );

    test_missingtitle!(
        multi_violation,
        r"
        contract Test {
            /// @title Some struct
            /// @title Some struct
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            MissingTitle::NAME,
            "Too many title comments".to_string(),
            sct.loc
        ))
    );

    test_missingtitle!(
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
            MissingTitle::NAME,
            "Missing a title comment".to_string(),
            sct.loc
        ))
    );

    test_missingtitle!(
        multiline_multi_violation,
        r"
        contract Test {
            /**
             * @title a Some struct
             * @title b Some struct
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            MissingTitle::NAME,
            "Too many title comments".to_string(),
            sct.loc
        ))
    );
}
