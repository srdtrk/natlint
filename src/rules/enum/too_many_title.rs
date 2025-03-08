use solang_parser::pt::EnumDefinition;

crate::too_many_comments_rule!(
    TooManyTitle,
    EnumDefinition,
    Title,
    "Enums must not have more than one title comment."
);

#[cfg(test)]
mod tests {
    use super::{EnumDefinition, TooManyTitle};
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

    macro_rules! test_too_many_title {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let item = child.as_enum().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);

                assert_eq!(TooManyTitle::check(Some(parent), item, comments), expected);
            }
        };
    }

    test_too_many_title!(
        empty_no_violation,
        r"
        interface Test {
            enum Option {
                Some,
                None
            }
        }
        ",
        |_| None
    );

    test_too_many_title!(
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

    test_too_many_title!(
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

    test_too_many_title!(
        multiline_no_violation,
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

    test_too_many_title!(
        missing_no_violation,
        r"
        contract Test {
            /// @custom:test Some comment
            enum Option {
                Some,
                None
            }
        }
        ",
        |_| None
    );

    test_too_many_title!(
        multiline_missing_no_violation,
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
        |_| None
    );

    test_too_many_title!(
        multi_violation,
        r"
        contract Test {
            /// @title Some enum
            /// @title Some enum
            enum Option {
                Some,
                None
            }
        }
        ",
        |sct: &EnumDefinition| Some(Violation::new(
            TooManyTitle::NAME,
            ViolationError::TooManyComments(CommentTag::Title),
            sct.loc
        ))
    );

    test_too_many_title!(
        multiline_multi_violation,
        r"
        contract Test {
            /**
             * @title a Some enum
             * @title b Some enum
             */
            enum Option {
                Some,
                None
            }
        }
        ",
        |sct: &EnumDefinition| Some(Violation::new(
            TooManyTitle::NAME,
            ViolationError::TooManyComments(CommentTag::Title),
            sct.loc
        ))
    );
}
