use solang_parser::pt::ErrorDefinition;

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that all errors have their parameters documented.
pub struct MissingParam;

impl Rule for MissingParam {
    type Target = ErrorDefinition;
    const NAME: &'static str = "MissingParam";
    const DESCRIPTION: &'static str = "Errors must document all parameters.";

    fn check(
        _: Option<&ParseItem>,
        item: &ErrorDefinition,
        comments: &CommentsRef,
    ) -> Option<Violation> {
        let param_comments = comments.include_tag(CommentTag::Param);
        match item.fields.len().cmp(&param_comments.len()) {
            std::cmp::Ordering::Less => {
                return Some(Violation::new(
                    Self::NAME,
                    Self::DESCRIPTION,
                    ViolationError::TooManyComments(CommentTag::Param),
                    item.loc,
                ))
            }
            std::cmp::Ordering::Greater => {
                return Some(Violation::new(
                    Self::NAME,
                    Self::DESCRIPTION,
                    ViolationError::MissingComment(CommentTag::Param),
                    item.loc,
                ))
            }
            std::cmp::Ordering::Equal => (),
        }

        for field in &item.fields {
            let Some(field_id) = field.name.as_ref() else {
                return Some(Violation::new(
                    Self::NAME,
                    Self::DESCRIPTION,
                    ViolationError::parse_error("Field name could not be parsed"),
                    field.loc,
                ));
            };

            if !param_comments.iter().any(|comment| {
                comment
                    .split_first_word()
                    .iter()
                    .map(|&(name, _)| name.to_owned())
                    .any(|content| content == field_id.name)
            }) {
                return Some(Violation::new(
                    Self::NAME,
                    Self::DESCRIPTION,
                    ViolationError::missing_comment_for(CommentTag::Param, &field_id.name),
                    field_id.loc,
                ));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, ErrorDefinition, MissingParam, Rule, Violation, ViolationError,
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
                let item = child.as_error().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);
                let result = MissingParam::check(None, item, &comments);

                assert_eq!(expected, result);
            }
        };
    }

    test_missingparams!(
        no_violation,
        r"
        interface Test {
            /// @param a Some param
            error Unauthorized(address a);
        }
        ",
        |_| None
    );

    test_missingparams!(
        empty_no_violation,
        r"
        interface Test {
            error Unauthorized();
        }
        ",
        |_| None
    );

    test_missingparams!(
        multi_no_violation,
        r"
        interface Test {
            /// @param a Some param
            /// @param b Some param
            error Unauthorized(address a, address b);
        }
        ",
        |_| None
    );

    test_missingparams!(
        multiline_no_violation,
        r"
        interface Test {
            /**
             * @param a Some param
             * @param b Some param
             */
            error Unauthorized(address a, address b);
        }
        ",
        |_| None
    );

    test_missingparams!(
        too_many_comments_violation,
        r"
        interface Test {
            /// @param a Some param
            /// @param b Some param
            error Unauthorized(address a);
        }
        ",
        |item: &ErrorDefinition| Some(Violation::new(
            MissingParam::NAME,
            MissingParam::DESCRIPTION,
            ViolationError::TooManyComments(CommentTag::Param),
            item.loc
        ))
    );

    test_missingparams!(
        multiline_too_many_comments_violation,
        r"
        interface Test {
            /**
             * @param a Some param
             * @param b Some param
             */
            error Unauthorized(address a);
        }
        ",
        |item: &ErrorDefinition| Some(Violation::new(
            MissingParam::NAME,
            MissingParam::DESCRIPTION,
            ViolationError::TooManyComments(CommentTag::Param),
            item.loc
        ))
    );

    test_missingparams!(
        empty_violation,
        r"
        interface Test {
            error Unauthorized(address a);
        }
        ",
        |item: &ErrorDefinition| Some(Violation::new(
            MissingParam::NAME,
            MissingParam::DESCRIPTION,
            ViolationError::MissingComment(CommentTag::Param),
            item.loc
        ))
    );

    test_missingparams!(
        missing_param_name_violation,
        r"
        interface Test {
            /// @param a Some param
            /// @param c Some param
            error Unauthorized(address a, address b);
        }
        ",
        |item: &ErrorDefinition| Some(Violation::new(
            MissingParam::NAME,
            MissingParam::DESCRIPTION,
            ViolationError::missing_comment_for(CommentTag::Param, "b"),
            item.fields[1].name.as_ref().unwrap().loc
        ))
    );

    test_missingparams!(
        multiline_missing_param_name_violation,
        r"
        interface Test {
            /**
             * @param a Some param
             * @param c Some param
             */
            error Unauthorized(address a, address b);
        }
        ",
        |item: &ErrorDefinition| Some(Violation::new(
            MissingParam::NAME,
            MissingParam::DESCRIPTION,
            ViolationError::missing_comment_for(CommentTag::Param, "b"),
            item.fields[1].name.as_ref().unwrap().loc
        ))
    );
}
