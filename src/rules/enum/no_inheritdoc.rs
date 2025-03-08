use solang_parser::pt::EnumDefinition;

crate::no_comment_rule!(
    NoInheritdoc,
    EnumDefinition,
    Inheritdoc,
    "Enums must not have an inheritdoc comment."
);

#[cfg(test)]
mod tests {
    use super::{EnumDefinition, NoInheritdoc};
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

    macro_rules! test_no_inheritdoc {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let item = child.as_enum().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);

                assert_eq!(NoInheritdoc::check(Some(parent), item, comments), expected);
            }
        };
    }

    test_no_inheritdoc!(
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

    test_no_inheritdoc!(
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
    test_no_inheritdoc!(
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

    test_no_inheritdoc!(
        inheritdoc_violation,
        r"
        interface Test {
            /// @inheritdoc Base
            enum Option {
                Some,
                None
            }
        }
        ",
        |item: &EnumDefinition| Some(Violation::new(
            NoInheritdoc::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Inheritdoc),
            item.loc
        ))
    );

    test_no_inheritdoc!(
        multiline_inheritdoc_violation,
        r"
        interface Test {
            /**
             * @inheritdoc Base
             */
            enum Option {
                Some,
                None
            }
        }
        ",
        |item: &EnumDefinition| Some(Violation::new(
            NoInheritdoc::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Inheritdoc),
            item.loc
        ))
    );
}
