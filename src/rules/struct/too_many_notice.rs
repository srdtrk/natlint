use solang_parser::pt::StructDefinition;

crate::too_many_comments_rule!(
    TooManyNotice,
    StructDefinition,
    Notice,
    "Structs must not have more than one notice comment."
);

#[cfg(test)]
mod tests {
    use super::{StructDefinition, TooManyNotice};
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

                assert_eq!(TooManyNotice::check(Some(parent), item, comments), expected);
            }
        };
    }

    test_missingnotice!(
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
        no_tag_violation,
        r"
        contract Test {
            /// Some notice
            /// @notice b Some struct
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            TooManyNotice::NAME,
            ViolationError::TooManyComments(CommentTag::Notice),
            sct.loc
        )) // WARNING: solang parser and the natspec docs interpret no tags as a notice
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
            TooManyNotice::NAME,
            ViolationError::TooManyComments(CommentTag::Notice),
            sct.loc
        ))
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
            TooManyNotice::NAME,
            ViolationError::TooManyComments(CommentTag::Notice),
            sct.loc
        ))
    );
}
