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
        _parent: Option<&ParseItem>,
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
