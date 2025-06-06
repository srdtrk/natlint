use solang_parser::pt::EnumDefinition;

crate::too_many_comments_rule!(
    TooManyNotice,
    EnumDefinition,
    Notice,
    "Enums must not have more than one notice comment."
);

#[cfg(test)]
mod tests {
    use super::{EnumDefinition, TooManyNotice};
    use crate::{
        generate_too_many_comment_test_cases,
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

    macro_rules! test_too_many_notice {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let item = child.as_enum().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);

                assert_eq!(
                    TooManyNotice::check(Some(parent), item, &comments),
                    expected
                );
            }
        };
    }

    generate_too_many_comment_test_cases!(
        Notice,
        test_too_many_notice,
        TooManyNotice,
        r"
            enum Option {
                Some,
                None
            }
        ",
        "@notice",
        EnumDefinition
    );

    test_too_many_notice!(
        no_tag_violation,
        r"
        contract Test {
            /// Some notice
            /// @notice Some other
            enum Option {
                Some,
                None
            }
        }
        ",
        |sct: &EnumDefinition| Some(Violation::new(
            TooManyNotice::NAME,
            TooManyNotice::DESCRIPTION,
            ViolationError::TooManyComments(CommentTag::Notice),
            sct.loc
        )) // WARNING: solang parser and the natspec docs interpret no tags as a notice
    );

    test_too_many_notice!(
        multiline_no_tag_violation,
        r"
        contract Test {
            /**
             * Some comment
             * @notice Some othe
             */
            enum Option {
                Some,
                None
            }
        }
        ",
        |sct: &EnumDefinition| Some(Violation::new(
            TooManyNotice::NAME,
            TooManyNotice::DESCRIPTION,
            ViolationError::TooManyComments(CommentTag::Notice),
            sct.loc
        )) // WARNING: solang parser and the natspec docs interpret no tags as a notice
    );
}
