use solang_parser::pt::{FunctionDefinition, FunctionTy};

use crate::parser::{CommentTag, CommentsRef, ParseItem};

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
                    "Too many param comments".to_string(),
                    func.loc,
                ));
            }
            std::cmp::Ordering::Greater => {
                return Some(Violation::new(
                    Self::NAME,
                    "Missing param or inheritdoc comment".to_string(),
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
                    format!("Missing param comment for `{param_name}`"),
                    *loc,
                ));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionDefinition, MissingParams, Rule};
    use crate::{
        parser::{CommentsRef, Parser},
        rules::Violation,
    };
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    /// Macro to define a test case for `MissingParams` rule
    macro_rules! test_require_missingparams {
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

    test_require_missingparams!(
        public_no_violation,
        r"
        contract Test {
            /// @param a A number
            function test(uint256 a) public {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        private_no_violation,
        r"
        contract Test {
            /// @param a A number
            function test(uint256 a) private {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        dollar_no_violation,
        r"
        contract Test {
            /// @param $ A number
            function test(uint256 $) private {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
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

    test_require_missingparams!(
        inheritdoc_no_violation,
        r"
        contract Test {
            /// @inheritdoc something
            function test(uint256 a) public {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        unnamed_no_violation,
        r"
        contract Test {
            /// @param A number
            function test(uint256) public {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
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

    test_require_missingparams!(
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

    test_require_missingparams!(
        public_violation,
        r"
        contract Test {
            /// @notice Some function
            function test(uint256 a) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingParams::NAME,
            "Missing param or inheritdoc comment".to_string(),
            func.loc
        ))
    );

    test_require_missingparams!(
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
            "Too many param comments".to_string(),
            func.loc
        ))
    );

    test_require_missingparams!(
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
            "Too many param comments".to_string(),
            func.loc
        ))
    );

    test_require_missingparams!(
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
            "Missing param comment for `b`".to_string(),
            func.params[1].0
        ))
    );

    test_require_missingparams!(
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
            "Missing param comment for `b`".to_string(),
            func.params[1].0
        ))
    );
}
