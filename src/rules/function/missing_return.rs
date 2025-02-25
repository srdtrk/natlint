use solang_parser::pt::{FunctionDefinition, FunctionTy};

use crate::parser::{CommentTag, CommentsRef, ParseItem};

use super::super::{Rule, Violation};

/// This rule requires that all functions have their return variables documented or have an inheritdoc comment.
pub struct MissingReturn;

impl Rule<FunctionDefinition> for MissingReturn {
    const NAME: &'static str = "Missing Return";
    const DESCRIPTION: &'static str =
        "All functions must have their return variables documented or have an inheritdoc comment.";

    fn check(
        _: Option<&ParseItem>,
        func: &FunctionDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // Function type must be a user function
        match func.ty {
            FunctionTy::Function => (),
            FunctionTy::Receive
            | FunctionTy::Fallback
            | FunctionTy::Modifier
            | FunctionTy::Constructor => return None,
        }

        // If the function has an inheritdoc comment, it is exempt from this rule
        if comments.find_inheritdoc_base().is_some() {
            return None;
        }

        // Function must have a return comment for each return variable
        let return_comments = comments.include_tag(CommentTag::Return);
        match func.returns.len().cmp(&return_comments.len()) {
            std::cmp::Ordering::Less => {
                return Some(Violation::new(
                    Self::NAME,
                    "Too many return comments".to_string(),
                    func.loc,
                ));
            }
            std::cmp::Ordering::Greater => {
                return Some(Violation::new(
                    Self::NAME,
                    "Missing return or inheritdoc comment".to_string(),
                    func.loc,
                ));
            }
            std::cmp::Ordering::Equal => (),
        }
        for (loc, return_var) in &func.returns {
            let Some(var_name) = return_var
                .as_ref()
                .and_then(|p| p.name.as_ref().map(|id| id.name.to_string()))
            else {
                // Skip unnamed parameters
                continue;
            };

            if !return_comments.iter().any(|comment| {
                comment
                    .split_first_word()
                    .map(|(name, _)| name.to_string())
                    .unwrap_or_default()
                    == var_name
            }) {
                return Some(Violation::new(
                    Self::NAME,
                    format!("Missing return comment for `{var_name}`"),
                    *loc,
                ));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionDefinition, MissingReturn, Rule};
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

                assert_eq!(MissingReturn::check(Some(parent), func, comments), expected);
            }
        };
    }

    test_require_missingparams!(
        no_return_no_violation,
        r"
        contract Test {
            function test(uint256) public {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        named_no_violation,
        r"
        contract Test {
            /// @return b A number
            function test(uint256) public returns (uint256 b) {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        unnamed_no_violation,
        r"
        contract Test {
            /// @return A number
            function test(uint256) public returns (uint256) {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        dollar_no_violation,
        r"
        contract Test {
            /// @return $ A number
            function test(uint256) public returns (uint256 $) {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        memory_no_violation,
        r"
        contract Test {
            /// @return b Some bytes
            function test(uint256) public returns (bytes memory b) {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        multiline_no_violation,
        r"
        contract Test {
            /**
             * @return b A number
             */
            function test(uint256) private returns (uint256 b) {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        inheritdoc_no_violation,
        r"
        contract Test {
            /// @inheritdoc something
            function test(uint256) public returns (uint256 b) {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        multiline_inheritdoc_no_violation,
        r"
        contract Test {
            /**
             * @inheritdoc something
             */
            function test(uint256) public returns (uint256 b) {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        multiple_no_violation,
        r"
        contract Test {
            /// @return a A number
            /// @return b Some string
            function test(uint256) public returns (uint256 a, string memory b) {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        multiple_multiline_no_violation,
        r"
        contract Test {
            /**
             * @return a A number
             * @return b Some string
             */
            function test(uint256) public returns (uint256 a, string memory b) {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        unnamed_multiple_no_violation,
        r"
        contract Test {
            /// @return a A number
            /// @return Some string
            function test(uint256) public returns (uint256 a, string memory) {}
        }
        ",
        |_| None
    );

    test_require_missingparams!(
        named_violation,
        r"
        contract Test {
            /// @notice Some function
            function test(uint256) public returns (uint256 a) {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingReturn::NAME,
            "Missing return or inheritdoc comment".to_string(),
            func.loc
        ))
    );

    test_require_missingparams!(
        unnamed_violation,
        r"
        contract Test {
            /// @notice Some function
            function test(uint256) public returns (uint256) {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingReturn::NAME,
            "Missing return or inheritdoc comment".to_string(),
            func.loc
        ))
    );

    test_require_missingparams!(
        too_many_comments_violation,
        r"
        contract Test {
            /// @return a A number
            /// @return b A number
            function test(uint256) public returns (uint256 a) {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingReturn::NAME,
            "Too many return comments".to_string(),
            func.loc
        ))
    );

    test_require_missingparams!(
        multiline_many_comments_violation,
        r"
        contract Test {
            /**
             * @return a A number
             * @return b A number
             */
            function test(uint256) public returns (uint256 a) {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingReturn::NAME,
            "Too many return comments".to_string(),
            func.loc
        ))
    );

    test_require_missingparams!(
        name_not_found_violation,
        r"
        contract Test {
            /// @return a A number
            /// @return c A number
            function test(uint256) public returns (uint256 a, uint256 b) {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingReturn::NAME,
            "Missing return comment for `b`".to_string(),
            func.returns[1].0
        ))
    );

    test_require_missingparams!(
        multiline_name_not_found_violation,
        r"
        contract Test {
            /**
             * @return a A number
             * @return c A number
             */
            function test(uint256) public returns (uint256 a, uint256 b) {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingReturn::NAME,
            "Missing return comment for `b`".to_string(),
            func.returns[1].0
        ))
    );
}
