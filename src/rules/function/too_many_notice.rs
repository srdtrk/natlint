use solang_parser::pt::FunctionDefinition;

crate::too_many_comments_rule!(
    TooManyNotice,
    FunctionDefinition,
    Notice,
    "Functions must not have more than one notice comment."
);

#[cfg(test)]
mod tests {
    use super::{FunctionDefinition, TooManyNotice};
    use crate::{
        parser::{CommentTag, CommentsRef, Parser},
        rules::{Rule, Violation, ViolationError},
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
                let func = child.as_function().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(func);

                assert_eq!(TooManyNotice::check(Some(parent), func, comments), expected);
            }
        };
    }

    test_missingnotice!(
        too_many_comments_violation,
        r"
        contract Test {
            /// @notice Some function
            /// @notice Another function
            function test(uint256 a) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            TooManyNotice::NAME,
            ViolationError::TooManyComments(CommentTag::Notice),
            func.loc
        ))
    );

    test_missingnotice!(
        too_many_comments_tag_no_tag_violation,
        r"
        contract Test {
            /// Another function
            /// @notice Some function
            function test(uint256 a) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            TooManyNotice::NAME,
            ViolationError::TooManyComments(CommentTag::Notice),
            func.loc
        ))
    );

    test_missingnotice!(
        multiline_many_comments_violation,
        r"
        contract Test {
            /**
             * @notice Some function
             * @notice Another function
             */
            function test(uint256 a) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            TooManyNotice::NAME,
            ViolationError::TooManyComments(CommentTag::Notice),
            func.loc
        ))
    );
}
