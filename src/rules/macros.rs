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

/// Generates test cases for a missing comment rule.
#[macro_export]
macro_rules! generate_missing_comment_tests {
    (
        $comment_variant:ident,         // The CommentTag variant (e.g., Author)
        $test_macro:ident,              // The test macro name (e.g., test_missingauthor)
        $rule_name:ident,               // The rule struct (e.g., MissingAuthor)
        $struct_body:expr,              // The body of the struct/enum/function being tested
        $doc_tag:expr,                  // The doc tag (e.g., "@author")
        $parse_item:ty                  // The parse item type (e.g., StructDefinition)
    ) => {
        $test_macro!(
            no_violation,
            &format!(
                r"
                contract Test {{
                    /// {} Some comment
                    {}
                }}",
                $doc_tag, $struct_body
            ),
            |_| None
        );

        $test_macro!(
            multi_no_violation,
            &format!(
                r"
                contract Test {{
                    /// {} Some comment
                    /// @custom:test Some comment
                    {}
                }}",
                $doc_tag, $struct_body
            ),
            |_| None
        );

        $test_macro!(
            multi_comment_no_violation,
            &format!(
                r"
                contract Test {{
                    /// {} Some comment
                    /// {} Some other
                    {}
                }}",
                $doc_tag, $doc_tag, $struct_body
            ),
            |_| None
        );

        $test_macro!(
            multiline_multi_no_violation,
            &format!(
                r"
                contract Test {{
                    /**
                     * {} Some comment
                     * @custom:test Some comment
                     */
                    {}
                }}",
                $doc_tag, $struct_body
            ),
            |_| None
        );

        $test_macro!(
            multiline_multi_comment_no_violation,
            &format!(
                r"
                contract Test {{
                    /**
                     * {} Some comment
                     * {} Some other
                     */
                    {}
                }}",
                $doc_tag, $doc_tag, $struct_body
            ),
            |_| None
        );

        $test_macro!(
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

        $test_macro!(
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

        $test_macro!(
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
    };
}

/// Generates test cases for a "No Comment" rule.
#[macro_export]
macro_rules! generate_no_comment_tests {
    (
        $comment_variant:ident,        // The CommentTag variant (e.g., Inheritdoc)
        $test_macro:ident,             // The test macro name (e.g., test_no_inheritdoc)
        $rule_name:ident,              // The rule struct (e.g., NoInheritdoc)
        $struct_body:expr,             // The body of the struct/enum/function being tested
        $doc_tag:expr,                 // The doc tag (e.g., "@inheritdoc")
        $parse_item:ty                 // The parse item type (e.g., StructDefinition)
    ) => {
        // Case: No violation when there is no comment
        $test_macro!(
            empty_no_violation,
            &format!(
                r"
                interface Test {{
                    {}
                }}",
                $struct_body
            ),
            |_| None
        );

        // Case: No violation when there is a different comment tag
        $test_macro!(
            no_violation,
            &format!(
                r"
                interface Test {{
                    /// @custom:test Some comment
                    {}
                }}",
                $struct_body
            ),
            |_| None
        );

        // Case: No violation when there is a different comment tag (multiline)
        $test_macro!(
            multiline_no_violation,
            &format!(
                r"
                interface Test {{
                    /**
                     * @custom:test Some comment
                     */
                    {}
                }}",
                $struct_body
            ),
            |_| None
        );

        // Case: Violation when the disallowed comment is present
        $test_macro!(
            comment_violation,
            &format!(
                r"
                interface Test {{
                    /// {} Some comment
                    {}
                }}",
                $doc_tag, $struct_body
            ),
            |item: &$parse_item| Some(Violation::new(
                $rule_name::NAME,
                ViolationError::CommentNotAllowed(CommentTag::$comment_variant),
                item.loc
            ))
        );

        // Case: Violation when the disallowed comment appears multiple times
        $test_macro!(
            multi_comment_violation,
            &format!(
                r"
                interface Test {{
                    /// {} Some comment
                    /// {} Some other
                    {}
                }}",
                $doc_tag, $doc_tag, $struct_body
            ),
            |item: &$parse_item| Some(Violation::new(
                $rule_name::NAME,
                ViolationError::CommentNotAllowed(CommentTag::$comment_variant),
                item.loc
            ))
        );

        // Case: Violation when both allowed and disallowed comments are present
        $test_macro!(
            with_comment_violation,
            &format!(
                r"
                interface Test {{
                    /// {} Some comment
                    /// @custom:test Some comment
                    {}
                }}",
                $doc_tag, $struct_body
            ),
            |item: &$parse_item| Some(Violation::new(
                $rule_name::NAME,
                ViolationError::CommentNotAllowed(CommentTag::$comment_variant),
                item.loc
            ))
        );

        // Case: Violation when the disallowed comment appears in multiline
        $test_macro!(
            multiline_comment_violation,
            &format!(
                r"
                interface Test {{
                    /**
                     * {} Some comment
                     */
                    {}
                }}",
                $doc_tag, $struct_body
            ),
            |item: &$parse_item| Some(Violation::new(
                $rule_name::NAME,
                ViolationError::CommentNotAllowed(CommentTag::$comment_variant),
                item.loc
            ))
        );

        // Case: Violation when both allowed and disallowed comments are present in multiline
        $test_macro!(
            multiline_with_comment_violation,
            &format!(
                r"
                interface Test {{
                    /**
                     * {} Some comment
                     * @custom:test Some comment
                     */
                    {}
                }}",
                $doc_tag, $struct_body
            ),
            |item: &$parse_item| Some(Violation::new(
                $rule_name::NAME,
                ViolationError::CommentNotAllowed(CommentTag::$comment_variant),
                item.loc
            ))
        );

        // Case: Violation when the disallowed comment appears multiple times in multiline
        $test_macro!(
            multiline_multi_comment_violation,
            &format!(
                r"
                interface Test {{
                    /**
                     * {} Some comment
                     * {} Some other
                     */
                    {}
                }}",
                $doc_tag, $doc_tag, $struct_body
            ),
            |item: &$parse_item| Some(Violation::new(
                $rule_name::NAME,
                ViolationError::CommentNotAllowed(CommentTag::$comment_variant),
                item.loc
            ))
        );
    };
}
