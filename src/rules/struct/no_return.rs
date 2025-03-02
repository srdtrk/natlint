use solang_parser::pt::StructDefinition;

crate::no_comment_rule!(
    NoReturn,
    StructDefinition,
    Return,
    "Structs must not have a return comment."
);

#[cfg(test)]
mod tests {
    use super::{NoReturn, StructDefinition};
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
                let item = child.as_struct().unwrap();
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
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_no_return!(
        no_violation,
        r"
        interface Test {
            /// @inheritdoc Base
            struct TestStruct {
                uint256 a;
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
             * @notice Some notice
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_no_return!(
        return_violation,
        r"
        interface Test {
            /// @return Some return
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |item: &StructDefinition| Some(Violation::new(
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
             * @return Some return
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |item: &StructDefinition| Some(Violation::new(
            NoReturn::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Return),
            item.loc
        ))
    );
}
