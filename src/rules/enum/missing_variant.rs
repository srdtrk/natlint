//! At the time of writing, solidity does not support a way to document enum variants.
//! This has led to different practices in the community, each with its own pros and cons.
//! This rule follows the practice of using `@custom:variant` to document enum variants.
//! This practice is used by `foundry-rs` to document enum variants. <https://github.com/foundry-rs/foundry/issues/9545>
//! This rule is disabled by default.

use solang_parser::pt::EnumDefinition;

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that enums do not miss any variants.
pub struct MissingVariant;

impl Rule for MissingVariant {
    type Target = EnumDefinition;
    const NAME: &'static str = "MissingVariant";
    const DESCRIPTION: &'static str = "Enums must document all variants.";

    fn check(
        _: Option<&ParseItem>,
        item: &EnumDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        let variant_comments = comments.include_tag(CommentTag::variant());
        match item.values.len().cmp(&variant_comments.len()) {
            std::cmp::Ordering::Less => {
                return Some(Violation::new(
                    Self::NAME,
                    ViolationError::TooManyComments(CommentTag::variant()),
                    item.loc,
                ))
            }
            std::cmp::Ordering::Greater => {
                return Some(Violation::new(
                    Self::NAME,
                    ViolationError::MissingComment(CommentTag::variant()),
                    item.loc,
                ))
            }
            std::cmp::Ordering::Equal => (),
        }

        for variant in &item.values {
            let Some(variant_id) = variant.as_ref() else {
                return Some(Violation::new(
                    Self::NAME,
                    ViolationError::parse_error("Variant name could not be parsed"),
                    item.loc,
                ));
            };

            if !variant_comments.iter().any(|comment| {
                comment
                    .split_first_word()
                    .iter()
                    .map(|&(name, _)| name.to_string())
                    .any(|content| content == variant_id.name)
            }) {
                return Some(Violation::new(
                    Self::NAME,
                    ViolationError::missing_comment_for(CommentTag::variant(), &variant_id.name),
                    variant_id.loc,
                ));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, EnumDefinition, MissingVariant, Rule, Violation, ViolationError,
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

    macro_rules! test_missingvariant {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let item = child.as_enum().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);
                let result = MissingVariant::check(None, item, comments);

                assert_eq!(expected, result);
            }
        };
    }

    test_missingvariant!(
        no_violation,
        r"
        interface Test {
            /// @custom:variant Some Some variant
            enum Option {
                Some
            }
        }
        ",
        |_| None
    );

    test_missingvariant!(
        empty_no_violation,
        r"
        interface Test {
            enum Option {
            }
        }
        ",
        |_| None
    );

    test_missingvariant!(
        multi_no_violation,
        r"
        interface Test {
            /// @custom:variant Some Some variant
            /// @custom:variant None Other variant
            enum Option {
                Some,
                None
            }
        }
        ",
        |_| None
    );

    test_missingvariant!(
        multiline_no_violation,
        r"
        interface Test {
            /**
             * @custom:variant Some Some variant
             * @custom:variant None Other variant
             */
            enum Option {
                Some,
                None
            }
        }
        ",
        |_| None
    );

    test_missingvariant!(
        too_many_comments_violation,
        r"
        interface Test {
            /// @custom:variant Some Some variant
            /// @custom:variant None Other variant
            enum Option {
                Some
            }
        }
        ",
        |item: &EnumDefinition| Some(Violation::new(
            MissingVariant::NAME,
            ViolationError::TooManyComments(CommentTag::variant()),
            item.loc
        ))
    );

    test_missingvariant!(
        multiline_too_many_comments_violation,
        r"
        interface Test {
            /**
             * @custom:variant Some Some variant
             * @custom:variant None Other variant
             */
            enum Option {
                Some
            }
        }
        ",
        |item: &EnumDefinition| Some(Violation::new(
            MissingVariant::NAME,
            ViolationError::TooManyComments(CommentTag::variant()),
            item.loc
        ))
    );

    test_missingvariant!(
        empty_violation,
        r"
        interface Test {
            enum Option {
                Some
            }
        }
        ",
        |item: &EnumDefinition| Some(Violation::new(
            MissingVariant::NAME,
            ViolationError::MissingComment(CommentTag::variant()),
            item.loc
        ))
    );

    test_missingvariant!(
        missing_param_name_violation,
        r"
        interface Test {
            /// @custom:variant Some Some variant
            /// @custom:variant wrong Wrong variant
            enum Option {
                Some,
                None
            }
        }
        ",
        |item: &EnumDefinition| Some(Violation::new(
            MissingVariant::NAME,
            ViolationError::missing_comment_for(CommentTag::variant(), "None"),
            item.values[1].as_ref().unwrap().loc
        ))
    );

    test_missingvariant!(
        multiline_missing_param_name_violation,
        r"
        interface Test {
            /**
             * @custom:variant Some Some variant
             * @custom:variant wrong Wrong variant
             */
            enum Option {
                Some,
                None
            }
        }
        ",
        |item: &EnumDefinition| Some(Violation::new(
            MissingVariant::NAME,
            ViolationError::missing_comment_for(CommentTag::variant(), "None"),
            item.values[1].as_ref().unwrap().loc
        ))
    );
}
