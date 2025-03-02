//! This module defines the macros for quickly defining some classes of rules.

/// This macro defines a rule that checks against a specific comment tag.
#[macro_export]
macro_rules! no_comment_rule {
    ($rule_name:ident, $parse_item:ty, $comment_variant:ident, $description:expr) => {
        #[doc = $description]
        pub struct $rule_name;

        impl $crate::rules::Rule<$parse_item> for $rule_name {
            const NAME: &'static str = stringify!($rule_name);
            const DESCRIPTION: &'static str = $description;

            fn check(
                _: Option<&$crate::parser::ParseItem>,
                item: &$parse_item,
                comments: $crate::parser::CommentsRef,
            ) -> Option<$crate::rules::Violation> {
                if !comments
                    .include_tag($crate::parser::CommentTag::$comment_variant)
                    .is_empty()
                {
                    return Some($crate::rules::Violation::new(
                        Self::NAME,
                        $crate::rules::violation_error::ViolationError::CommentNotAllowed(
                            $crate::parser::CommentTag::$comment_variant,
                        ),
                        item.loc,
                    ));
                }
                None
            }
        }
    };
}
