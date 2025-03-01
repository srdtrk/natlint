use solang_parser::pt::{FunctionDefinition, FunctionTy};

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that all functions have their parameters documented or have an inheritdoc
/// comment.
pub struct MissingParams;

impl Rule<FunctionDefinition> for MissingParams {
    const NAME: &'static str = "Missing Params";
    const DESCRIPTION: &'static str =
        "All functions must have their parameters documented or have an inheritdoc comment.";

    fn check(
        _: Option<&ParseItem>,
        func: &FunctionDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // Function must not be a modifier or constructor
        match func.ty {
            FunctionTy::Function | FunctionTy::Constructor | FunctionTy::Modifier => (),
            FunctionTy::Receive | FunctionTy::Fallback => return None,
        }

        // If the function has an inheritdoc comment, it is exempt from this rule
        if comments.find_inheritdoc_base().is_some() {
            return None;
        }

        // Function must have a parameter comment for each parameter
        let param_comments = comments.include_tag(CommentTag::Param);
        match func.params.len().cmp(&param_comments.len()) {
            std::cmp::Ordering::Less => {
                return Some(Violation::new(
                    Self::NAME,
                    ViolationError::TooManyComments(CommentTag::Param),
                    func.loc,
                ));
            }
            std::cmp::Ordering::Greater => {
                return Some(Violation::new(
                    Self::NAME,
                    ViolationError::MissingComment(CommentTag::Param),
                    func.loc,
                ));
            }
            std::cmp::Ordering::Equal => (),
        }
        for (loc, param) in &func.params {
            let Some(param_name) = param
                .as_ref()
                .and_then(|p| p.name.as_ref().map(|id| id.name.to_string()))
            else {
                // Skip unnamed parameters
                continue;
            };

            if !param_comments.iter().any(|comment| {
                comment
                    .split_first_word()
                    .map(|(name, _)| name.to_string())
                    .unwrap_or_default()
                    == param_name
            }) {
                return Some(Violation::new(
                    Self::NAME,
                    ViolationError::missing_comment_for(CommentTag::Param, &param_name),
                    *loc,
                ));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, FunctionDefinition, MissingParams, Rule, Violation, ViolationError,
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

    macro_rules! test_missingparams {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let func = child.as_function().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(func);

                assert_eq!(MissingParams::check(Some(parent), func, comments), expected);
            }
        };
    }

    test_missingparams!(
        no_params_no_violation,
        r"
        contract Test {
            function test() public {}
        }
        ",
        |_| None
    );

    test_missingparams!(
        public_no_violation,
        r"
        contract Test {
            /// @param a A number
            function test(uint256 a) public {}
        }
        ",
        |_| None
    );

    test_missingparams!(
        private_no_violation,
        r"
        contract Test {
            /// @param a A number
            function test(uint256 a) private {}
        }
        ",
        |_| None
    );

    test_missingparams!(
        dollar_no_violation,
        r"
        contract Test {
            /// @param $ A number
            function test(uint256 $) private {}
        }
        ",
        |_| None
    );

    test_missingparams!(
        multiline_no_violation,
        r"
        contract Test {
            /**
             * @param a A number
             */
            function test(uint256 a) private {}
        }
        ",
        |_| None
    );

    test_missingparams!(
        inheritdoc_no_violation,
        r"
        contract Test {
            /// @inheritdoc something
            function test(uint256 a) public {}
        }
        ",
        |_| None
    );

    test_missingparams!(
        multiline_inheritdoc_no_violation,
        r"
        contract Test {
            /**
             * @inheritdoc something
             */
            function test(uint256 a) public {}
        }
        ",
        |_| None
    );

    test_missingparams!(
        unnamed_no_violation,
        r"
        contract Test {
            /// @param A number
            function test(uint256) public {}
        }
        ",
        |_| None
    );

    test_missingparams!(
        multiple_no_violation,
        r"
        contract Test {
            /// @param a A number
            /// @param lol A string
            function test(uint256 a, string memory lol) public {}
        }
        ",
        |_| None
    );

    test_missingparams!(
        unnamed_multiple_no_violation,
        r"
        contract Test {
            /// @param a A number
            /// @param A string
            function test(uint256 a, string memory) public {}
        }
        ",
        |_| None
    );

    test_missingparams!(
        public_violation,
        r"
        contract Test {
            /// @notice Some function
            function test(uint256 a) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingParams::NAME,
            ViolationError::MissingComment(CommentTag::Param),
            func.loc
        ))
    );

    test_missingparams!(
        too_many_comments_violation,
        r"
        contract Test {
            /// @param a A number
            /// @param b A number
            function test(uint256 a) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingParams::NAME,
            ViolationError::TooManyComments(CommentTag::Param),
            func.loc
        ))
    );

    test_missingparams!(
        no_params_violation,
        r"
        contract Test {
            /// @param a A number
            function test() public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingParams::NAME,
            ViolationError::TooManyComments(CommentTag::Param),
            func.loc
        ))
    );

    test_missingparams!(
        multiline_many_comments_violation,
        r"
        contract Test {
            /**
             * @param a A number
             * @param b A number
             */
            function test(uint256 a) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingParams::NAME,
            ViolationError::TooManyComments(CommentTag::Param),
            func.loc
        ))
    );

    test_missingparams!(
        name_not_found_violation,
        r"
        contract Test {
            /// @param a A number
            /// @param c A number
            function test(uint256 a, uint256 b) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingParams::NAME,
            ViolationError::missing_comment_for(CommentTag::Param, "b"),
            func.params[1].0
        ))
    );

    test_missingparams!(
        multiline_name_not_found_violation,
        r"
        contract Test {
            /**
             * @param a A number
             * @param c A number
             */
            function test(uint256 a, uint256 b) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingParams::NAME,
            ViolationError::missing_comment_for(CommentTag::Param, "b"),
            func.params[1].0
        ))
    );
}
