use solang_parser::pt::StructDefinition;

use crate::parser::{CommentTag, CommentsRef, ParseItem};

use super::super::{Rule, Violation};

/// This rule requires that structs do not miss any parameters.
pub struct MissingParams;

impl Rule<StructDefinition> for MissingParams {
    const NAME: &'static str = "Missing Params";
    const DESCRIPTION: &'static str = "All structs must document all parameters.";

    fn check(
        _: Option<&ParseItem>,
        item: &StructDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        let param_comments = comments.include_tag(CommentTag::Param);
        match item.fields.len().cmp(&param_comments.len()) {
            std::cmp::Ordering::Less => {
                return Some(Violation::new(
                    Self::NAME,
                    "Too many param comments".to_string(),
                    item.loc,
                ))
            }
            std::cmp::Ordering::Greater => {
                return Some(Violation::new(
                    Self::NAME,
                    "Missing param comment".to_string(),
                    item.loc,
                ))
            }
            std::cmp::Ordering::Equal => (),
        }

        for field in &item.fields {
            let Some(field_id) = field.name.as_ref() else {
                return Some(Violation::new(
                    Self::NAME,
                    "Cannot parse paramater name".to_string(),
                    field.loc,
                ));
            };

            if !param_comments.iter().any(|comment| {
                comment
                    .split_first_word()
                    .iter()
                    .map(|&(name, _)| name.to_string())
                    .any(|content| content == field_id.name)
            }) {
                return Some(Violation::new(
                    Self::NAME,
                    format!("Missing param comment for field `{}`", field_id.name),
                    field_id.loc,
                ));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{MissingParams, Rule, StructDefinition};
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
    macro_rules! test_missingparams {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let item = child.as_struct().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);
                let result = MissingParams::check(None, item, comments);

                assert_eq!(expected, result);
            }
        };
    }

    test_missingparams!(
        no_violation,
        r"
        interface Test {
            /// @param a Some param
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_missingparams!(
        empty_no_violation,
        r"
        interface Test {
            struct TestStruct {
            }
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
            struct TestStruct {
                uint256 a;
                uint256 b;
            }
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
            struct TestStruct {
                uint256 a;
                uint256 b;
            }
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
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |item: &StructDefinition| Some(Violation::new(
            MissingParams::NAME,
            "Too many param comments".to_string(),
            item.loc
        ))
    );

    test_missingparams!(
        empty_violation,
        r"
        interface Test {
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |item: &StructDefinition| Some(Violation::new(
            MissingParams::NAME,
            "Missing param comment".to_string(),
            item.loc
        ))
    );

    test_missingparams!(
        missing_param_name_violation,
        r"
        interface Test {
            /// @param a Some param
            /// @param c Some param
            struct TestStruct {
                uint256 a;
                uint256 b;
            }
        }
        ",
        |item: &StructDefinition| Some(Violation::new(
            MissingParams::NAME,
            "Missing param comment for field `b`".to_string(),
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
            struct TestStruct {
                uint256 a;
                uint256 b;
            }
        }
        ",
        |item: &StructDefinition| Some(Violation::new(
            MissingParams::NAME,
            "Missing param comment for field `b`".to_string(),
            item.fields[1].name.as_ref().unwrap().loc
        ))
    );
}
