//! This rule requires that all enums have a title comment.
//! This rule will be off by default.

use solang_parser::pt::EnumDefinition;

crate::missing_comment_rule!(
    MissingTitle,
    EnumDefinition,
    Title,
    "Enums must have a title comment."
);

#[cfg(test)]
mod tests {
    use super::{EnumDefinition, MissingTitle};
    use crate::{
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

    macro_rules! test_missingtitle {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let item = child.as_enum().unwrap();

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
            enum Option {
                Some,
                None
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
            /// @custom:test Some comment
            enum Option {
                Some,
                None
            }
        }
        ",
        |_| None
    );

    test_missingtitle!(
        multi_title_no_violation,
        r"
        interface Test {
            /// @title Some title
            /// @title Some other
            enum Option {
                Some,
                None
            }
        }
        ",
        |_| None
    );

    test_missingtitle!(
        multiline_multi_no_violation,
        r"
        interface Test {
            /**
             * @title Some title
             * @custom:test Some comment
             */
            enum Option {
                Some,
                None
            }
        }
        ",
        |_| None
    );

    test_missingtitle!(
        multiline_multi_title_no_violation,
        r"
        interface Test {
            /**
             * @title Some title
             * @title Some other
             */
            enum Option {
                Some,
                None
            }
        }
        ",
        |_| None
    );

    test_missingtitle!(
        empty_violation,
        r"
        contract Test {
            enum Option {
                Some,
                None
            }
        }
        ",
        |sct: &EnumDefinition| Some(Violation::new(
            MissingTitle::NAME,
            ViolationError::MissingComment(CommentTag::Title),
            sct.loc
        ))
    );

    test_missingtitle!(
        violation,
        r"
        contract Test {
            /// @custom:test Some comment
            enum Option {
                Some,
                None
            }
        }
        ",
        |sct: &EnumDefinition| Some(Violation::new(
            MissingTitle::NAME,
            ViolationError::MissingComment(CommentTag::Title),
            sct.loc
        ))
    );

    test_missingtitle!(
        multiline_violation,
        r"
        contract Test {
            /**
             * @custom:test Some comment
             */
            enum Option {
                Some,
                None
            }
        }
        ",
        |sct: &EnumDefinition| Some(Violation::new(
            MissingTitle::NAME,
            ViolationError::MissingComment(CommentTag::Title),
            sct.loc
        ))
    );
}
