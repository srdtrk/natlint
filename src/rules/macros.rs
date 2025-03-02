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

/// This macro defines a rule that enforces the presence of a specific comment tag.
#[macro_export]
macro_rules! missing_comment_rule {
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
                if comments
                    .include_tag($crate::parser::CommentTag::$comment_variant)
                    .is_empty()
                {
                    return Some($crate::rules::Violation::new(
                        Self::NAME,
                        $crate::rules::violation_error::ViolationError::MissingComment(
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

/// This macro defines a rule that enforces a maximum of one comment with a specific tag.
#[macro_export]
macro_rules! too_many_comments_rule {
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
                if comments
                    .include_tag($crate::parser::CommentTag::$comment_variant)
                    .len()
                    > 1
                {
                    return Some($crate::rules::Violation::new(
                        Self::NAME,
                        $crate::rules::violation_error::ViolationError::TooManyComments(
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

/// This macro generates test cases for rules that check for missing comments.
/// NOTE: This macro will not work with contracts
#[macro_export]
macro_rules! missing_comment_rule_tests {
    ($rule_name:ident, $parse_item:ty, $comment_variant:ident, $doc_tag:expr, $struct_body:expr) => {
        #[cfg(test)]
        mod tests {
            use super::$rule_name;
            use forge_fmt::Visitable;
            use solang_parser::parse;
            use $crate::{
                parser::{CommentTag, CommentsRef, Parser},
                rules::{violation_error::ViolationError, Rule, Violation},
            };

            fn parse_source(src: &str) -> Parser {
                let (mut source, comments) = parse(src, 0).expect("failed to parse source");
                let mut doc = Parser::new(comments, src.to_owned());
                source.visit(&mut doc).expect("failed to visit source");
                doc
            }

            macro_rules! test_rule {
                ($name:ident, $source:expr, $expected:expr) => {
                    #[test]
                    fn $name() {
                        let src = parse_source($source);

                        let parent = src.items_ref().first().unwrap();
                        let child = parent.children.first().unwrap();
                        let item = child.as_ref().downcast_ref::<$parse_item>().unwrap();
                        let comments = CommentsRef::from(&child.comments);

                        let expected = $expected(item);

                        assert_eq!($rule_name::check(Some(parent), item, comments), expected);
                    }
                };
            }

            test_rule!(
                no_violation,
                &format!(
                    r"
                    interface Test {{
                        /// @{}
                        {}
                    }}",
                    $doc_tag, $struct_body
                ),
                |_| None
            );

            test_rule!(
                multi_no_violation,
                &format!(
                    r"
                    interface Test {{
                        /// @{}
                        /// @custom:test Some comment
                        {}
                    }}",
                    $doc_tag, $struct_body
                ),
                |_| None
            );

            test_rule!(
                multi_author_no_violation,
                &format!(
                    r"
                    interface Test {{
                        /// @{}
                        /// @{} Some other
                        {}
                    }}",
                    $doc_tag, $doc_tag, $struct_body
                ),
                |_| None
            );

            test_rule!(
                multiline_multi_no_violation,
                &format!(
                    r"
                    interface Test {{
                        /**
                         * @{}
                         * @custom:test Some comment
                         */
                        {}
                    }}",
                    $doc_tag, $struct_body
                ),
                |_| None
            );

            test_rule!(
                multiline_multi_author_no_violation,
                &format!(
                    r"
                    interface Test {{
                        /**
                         * @{}
                         * @{} Some other
                         */
                        {}
                    }}",
                    $doc_tag, $doc_tag, $struct_body
                ),
                |_| None
            );

            test_rule!(
                empty_violation,
                &format!(
                    r"
                    contract Test {{
                        {}
                    }}",
                    $struct_body
                ),
                |item: &$parse_item| Some(Violation::new(
                    $rule_name::NAME,
                    ViolationError::MissingComment(CommentTag::$comment_variant),
                    item.loc
                ))
            );

            test_rule!(
                violation,
                &format!(
                    r"
                    contract Test {{
                        /// @custom:test Some comment
                        {}
                    }}",
                    $struct_body
                ),
                |item: &$parse_item| Some(Violation::new(
                    $rule_name::NAME,
                    ViolationError::MissingComment(CommentTag::$comment_variant),
                    item.loc
                ))
            );

            test_rule!(
                multiline_violation,
                &format!(
                    r"
                    contract Test {{
                        /**
                         * @custom:test Some comment
                         */
                        {}
                    }}",
                    $struct_body
                ),
                |item: &$parse_item| Some(Violation::new(
                    $rule_name::NAME,
                    ViolationError::MissingComment(CommentTag::$comment_variant),
                    item.loc
                ))
            );
        }
    };
}
