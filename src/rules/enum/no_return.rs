use solang_parser::pt::EnumDefinition;

crate::no_comment_rule!(
    NoReturn,
    EnumDefinition,
    Return,
    "Enums must not have a return comment."
);

#[cfg(test)]
mod tests {
    use super::{EnumDefinition, NoReturn};
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

    macro_rules! test_no_return {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let item = child.as_enum().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);

                assert_eq!(NoReturn::check(Some(parent), item, comments), expected);
            }
        };
    }

    test_no_return!(
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

    test_no_return!(
        no_violation,
        r"
        interface Test {
            /// @custom:test Some comment
            enum Option {
                Some,
                None
            }
        }
        ",
        |_| None
    );
    test_no_return!(
        multiline_no_violation,
        r"
        interface Test {
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

    test_no_return!(
        return_violation,
        r"
        interface Test {
            /// @return something
            enum Option {
                Some,
                None
            }
        }
        ",
        |item: &EnumDefinition| Some(Violation::new(
            NoReturn::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Return),
            item.loc
        ))
    );

    test_no_return!(
        multiline_return_violation,
        r"
        interface Test {
            /**
             * @return something
             */
            enum Option {
                Some,
                None
            }
        }
        ",
        |item: &EnumDefinition| Some(Violation::new(
            NoReturn::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Return),
            item.loc
        ))
    );
}
