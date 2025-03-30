//! This module defines the macros for quickly defining some classes of rules.

/// This macro defines a rule that checks against a specific comment tag.
#[macro_export]
macro_rules! no_comment_rule {
    ($rule_name:ident, $parse_item:ty, $comment_variant:ident, $description:expr) => {
        #[doc = $description]
        #[derive(Default, Debug)]
        pub struct $rule_name;

        impl $crate::rules::Rule for $rule_name {
            type Target = $parse_item;
            const NAME: &'static str = stringify!($rule_name);
            const DESCRIPTION: &'static str = $description;

            fn check(
                _: Option<&$crate::parser::ParseItem>,
                item: &Self::Target,
                comments: &$crate::parser::CommentsRef,
            ) -> Option<$crate::rules::Violation> {
                if !comments
                    .include_tag($crate::parser::CommentTag::$comment_variant)
                    .is_empty()
                {
                    return Some($crate::rules::Violation::new(
                        Self::NAME,
                        Self::DESCRIPTION,
                        $crate::rules::violation_error::ViolationError::CommentNotAllowed(
                            $crate::parser::CommentTag::$comment_variant,
                        ),
                        item.loc,
                    ));
                }
                None
            }
        }

        $crate::rule_serialize_deserialize_bool!($rule_name);
    };
}

/// This macro defines a rule that enforces the presence of a specific comment tag.
#[macro_export]
macro_rules! missing_comment_rule {
    ($rule_name:ident, $parse_item:ty, $comment_variant:ident, $description:expr) => {
        #[doc = $description]
        #[derive(Default, Debug)]
        pub struct $rule_name;

        impl $crate::rules::Rule for $rule_name {
            type Target = $parse_item;
            const NAME: &'static str = stringify!($rule_name);
            const DESCRIPTION: &'static str = $description;

            fn check(
                _: Option<&$crate::parser::ParseItem>,
                item: &Self::Target,
                comments: &$crate::parser::CommentsRef,
            ) -> Option<$crate::rules::Violation> {
                if comments
                    .include_tag($crate::parser::CommentTag::$comment_variant)
                    .is_empty()
                {
                    return Some($crate::rules::Violation::new(
                        Self::NAME,
                        Self::DESCRIPTION,
                        $crate::rules::violation_error::ViolationError::MissingComment(
                            $crate::parser::CommentTag::$comment_variant,
                        ),
                        item.loc,
                    ));
                }
                None
            }
        }

        $crate::rule_serialize_deserialize_bool!($rule_name);
    };
}

/// This macro defines a rule that enforces a maximum of one comment with a specific tag.
#[macro_export]
macro_rules! too_many_comments_rule {
    ($rule_name:ident, $parse_item:ty, $comment_variant:ident, $description:expr) => {
        #[doc = $description]
        #[derive(Default, Debug)]
        pub struct $rule_name;

        impl $crate::rules::Rule for $rule_name {
            type Target = $parse_item;
            const NAME: &'static str = stringify!($rule_name);
            const DESCRIPTION: &'static str = $description;

            fn check(
                _: Option<&$crate::parser::ParseItem>,
                item: &Self::Target,
                comments: &$crate::parser::CommentsRef,
            ) -> Option<$crate::rules::Violation> {
                if comments
                    .include_tag($crate::parser::CommentTag::$comment_variant)
                    .len()
                    > 1
                {
                    return Some($crate::rules::Violation::new(
                        Self::NAME,
                        Self::DESCRIPTION,
                        $crate::rules::violation_error::ViolationError::TooManyComments(
                            $crate::parser::CommentTag::$comment_variant,
                        ),
                        item.loc,
                    ));
                }
                None
            }
        }

        $crate::rule_serialize_deserialize_bool!($rule_name);
    };
}

/// Generates a serde helper module named `serde_logic` alongside the rule struct.
/// This module handles serializing/deserializing the rule struct
/// between `bool` in TOML and `Option<RuleStruct>` in Rust.
///
/// The provided rule struct must implement `Default`.
#[macro_export]
macro_rules! rule_serialize_deserialize_bool {
    ($rule_name:ident) => {
        // Define the helper module directly here with a fixed name "serde_logic".
        // This module will be created in the same scope as the $rule_name struct.
        pub mod serde_logic {
            use serde::{Deserialize, Deserializer, Serializer};
            // Use 'super' to access the rule struct defined in the parent scope.
            use super::$rule_name;

            /// Serializes `Option<RuleType>` to a boolean (`true` if Some, `false` if None).
            #[allow(dead_code, clippy::ref_option)]
            pub fn serialize<S>(
                value: &Option<$rule_name>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_bool(value.is_some())
            }

            /// Deserializes a boolean to `Option<RuleType>`.
            /// `true` becomes `Some(RuleType::default())`.
            /// `false` becomes `None`.
            /// Requires that `RuleType` implements `Default`.
            #[allow(dead_code)]
            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<$rule_name>, D::Error>
            where
                D: Deserializer<'de>,
                // Constraint on the specific rule type from the parent scope.
                $rule_name: Default,
            {
                let is_enabled = bool::deserialize(deserializer)?;
                if is_enabled {
                    Ok(Some($rule_name::default()))
                } else {
                    Ok(None)
                }
            }
        }
    };
}

/// This macro defines a config struct
#[macro_export]
macro_rules! define_rules_config {
    (
        $(#[$struct_attr:meta])*
        $vis:vis struct $StructName:ident {
            $( // Rule fields repetition block
                $(#[$rule_attr:meta])* // 0+ attributes for the rule field
                $rule_vis:vis $rule_name:ident : Option<$RuleType:ty>,
            )*
        }
    ) => {
        $(#[$struct_attr])*
        $vis struct $StructName {
            // Define the rule fields (implicitly Option<T> and serde(default))
            $(
                $(#[$rule_attr])*
                #[serde(default = "crate::rules::some_rule")]
                $rule_vis $rule_name : Option<$RuleType>,
            )*
        }

        // 2. Implement the rule_set function
        impl $StructName {
            /// Creates a Vec of boxed dynamic rules based on the configuration.
            ///
            /// Only includes rules where the corresponding field in the macro invocation is `Some`.
            /// Assumes `DynRule` trait is in scope.
            #[must_use]
            pub fn rule_set(&self) -> ::std::vec::Vec<::std::boxed::Box<dyn $crate::rules::DynRule>> {
                let mut rules: ::std::vec::Vec<::std::boxed::Box<dyn $crate::rules::DynRule>> = ::std::vec::Vec::new();

                $(
                    if self.$rule_name.is_some() {
                        rules.push(::std::boxed::Box::new(<$RuleType as ::std::default::Default>::default()));
                    }
                )*

                rules
            }
        }

        // 3. Implement Default (all rules set to true)
        impl ::std::default::Default for $StructName {
            fn default() -> Self {
                Self {
                    $(
                        $rule_name: Some(<$RuleType as ::std::default::Default>::default()),
                    )*
                }
            }
        }
    };
}

/// Generates test cases for a missing comment rule.
#[macro_export]
macro_rules! generate_missing_comment_test_cases {
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
                $rule_name::DESCRIPTION,
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
                $rule_name::DESCRIPTION,
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
                $rule_name::DESCRIPTION,
                ViolationError::MissingComment(CommentTag::$comment_variant),
                item.loc
            ))
        );
    };
}

/// Generates test cases for a "No Comment" rule.
#[macro_export]
macro_rules! generate_no_comment_test_cases {
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
                $rule_name::DESCRIPTION,
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
                $rule_name::DESCRIPTION,
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
                $rule_name::DESCRIPTION,
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
                $rule_name::DESCRIPTION,
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
                $rule_name::DESCRIPTION,
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
                $rule_name::DESCRIPTION,
                ViolationError::CommentNotAllowed(CommentTag::$comment_variant),
                item.loc
            ))
        );
    };
}

/// Generates test cases for a "Too Many Comment" rule.
#[macro_export]
macro_rules! generate_too_many_comment_test_cases {
    (
        $comment_variant:ident,        // The CommentTag variant (e.g., Title)
        $test_macro:ident,             // The test macro name (e.g., test_too_many_title)
        $rule_name:ident,              // The rule struct (e.g., TooManyTitle)
        $struct_body:expr,             // The body of the struct/enum/function being tested
        $doc_tag:expr,                 // The doc tag (e.g., "@title")
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

        // Case: No violation when there is exactly one of the required comment
        $test_macro!(
            no_violation,
            &format!(
                r"
                interface Test {{
                    /// {} Some comment
                    {}
                }}",
                $doc_tag, $struct_body
            ),
            |_| None
        );

        // Case: No violation when there is an allowed additional comment
        $test_macro!(
            multi_no_violation,
            &format!(
                r"
                interface Test {{
                    /// {} Some comment
                    /// @custom:test Some comment
                    {}
                }}",
                $doc_tag, $struct_body
            ),
            |_| None
        );

        // Case: No violation when the allowed additional comment appears in multiline
        $test_macro!(
            multiline_no_violation,
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
            |_| None
        );

        // Case: No violation when the required comment is missing but another is present
        $test_macro!(
            missing_no_violation,
            &format!(
                r"
                contract Test {{
                    /// @custom:test Some comment
                    {}
                }}",
                $struct_body
            ),
            |_| None
        );

        // Case: No violation when the required comment is missing but another is present (multiline)
        $test_macro!(
            multiline_missing_no_violation,
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
            |_| None
        );

        // Case: Violation when the disallowed comment appears multiple times
        $test_macro!(
            multi_violation,
            &format!(
                r"
                contract Test {{
                    /// {} Some comment
                    /// {} Some other
                    {}
                }}",
                $doc_tag, $doc_tag, $struct_body
            ),
            |item: &$parse_item| Some(Violation::new(
                $rule_name::NAME,
                $rule_name::DESCRIPTION,
                ViolationError::TooManyComments(CommentTag::$comment_variant),
                item.loc
            ))
        );

        // Case: Violation when the disallowed comment appears multiple times in multiline
        $test_macro!(
            multiline_multi_violation,
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
            |item: &$parse_item| Some(Violation::new(
                $rule_name::NAME,
                $rule_name::DESCRIPTION,
                ViolationError::TooManyComments(CommentTag::$comment_variant),
                item.loc
            ))
        );
    };
}
