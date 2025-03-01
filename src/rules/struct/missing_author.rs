//! This rule requires that all structs have an author comment.
//! This rule will be off by default.

use solang_parser::pt::StructDefinition;

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that all structs have an author comment.
pub struct MissingAuthor;

impl Rule<StructDefinition> for MissingAuthor {
    const NAME: &'static str = "Missing Author";
    const DESCRIPTION: &'static str = "This rule requires that all structs have an author comment.";

    fn check(
        _: Option<&ParseItem>,
        item: &StructDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // Struct must have a author comment
        match comments.include_tag(CommentTag::Author).len() {
            0 => Some(Violation::new(
                Self::NAME,
                ViolationError::MissingComment(CommentTag::Author),
                item.loc,
            )),
            1 => None,
            _ => Some(Violation::new(
                Self::NAME,
                ViolationError::TooManyComments(CommentTag::Author),
                item.loc,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, MissingAuthor, Rule, StructDefinition, Violation, ViolationError,
    };
    use crate::parser::Parser;
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    macro_rules! test_missingauthor {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let item = child.as_struct().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);

                assert_eq!(MissingAuthor::check(Some(parent), item, comments), expected);
            }
        };
    }

    test_missingauthor!(
        no_violation,
        r"
        interface Test {
            /// @author Some author
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_missingauthor!(
        multi_no_violation,
        r"
        interface Test {
            /// @author Some author
            /// @param a Some param
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_missingauthor!(
        multiline_no_violation,
        r"
        interface Test {
            /**
             * @author Some author
             * @param a Some param
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_missingauthor!(
        empty_violation,
        r"
        contract Test {
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            MissingAuthor::NAME,
            ViolationError::MissingComment(CommentTag::Author),
            sct.loc
        ))
    );

    test_missingauthor!(
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
            MissingAuthor::NAME,
            ViolationError::MissingComment(CommentTag::Author),
            sct.loc
        ))
    );

    test_missingauthor!(
        multi_violation,
        r"
        contract Test {
            /// @author Some struct
            /// @author Some struct
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            MissingAuthor::NAME,
            ViolationError::TooManyComments(CommentTag::Author),
            sct.loc
        ))
    );

    test_missingauthor!(
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
            MissingAuthor::NAME,
            ViolationError::MissingComment(CommentTag::Author),
            sct.loc
        ))
    );

    test_missingauthor!(
        multiline_multi_violation,
        r"
        contract Test {
            /**
             * @author a Some struct
             * @author b Some struct
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |sct: &StructDefinition| Some(Violation::new(
            MissingAuthor::NAME,
            ViolationError::TooManyComments(CommentTag::Author),
            sct.loc
        ))
    );
}
