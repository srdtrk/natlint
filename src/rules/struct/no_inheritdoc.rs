use solang_parser::pt::StructDefinition;

use crate::parser::{CommentsRef, ParseItem};

use super::super::{Rule, Violation};

/// This rule requires that all structs must not have an inheritdoc comment.
pub struct NoInheritdoc;

impl Rule<StructDefinition> for NoInheritdoc {
    const NAME: &'static str = "No Inheritdoc";
    const DESCRIPTION: &'static str = "All structs must not have an inheritdoc comment.";

    fn check(
        _: Option<&ParseItem>,
        item: &StructDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        if comments.find_inheritdoc_base().is_some() {
            return Some(Violation::new(
                Self::NAME,
                Self::DESCRIPTION.to_string(),
                item.loc,
            ));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{NoInheritdoc, Rule, StructDefinition};
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
    macro_rules! test_no_inheritdoc {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let item = child.as_struct().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);

                assert_eq!(NoInheritdoc::check(Some(parent), item, comments), expected);
            }
        };
    }

    test_no_inheritdoc!(
        empty_no_violation,
        r"
        interface Test {
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_no_inheritdoc!(
        no_violation,
        r"
        interface Test {
            /// @notice Some notice
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );
    test_no_inheritdoc!(
        multiline_no_violation,
        r"
        interface Test {
            /**
             * @notice Some notice
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |_| None
    );

    test_no_inheritdoc!(
        inheritdoc_violation,
        r"
        interface Test {
            /// @inheritdoc Base
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |item: &StructDefinition| Some(Violation::new(
            NoInheritdoc::NAME,
            NoInheritdoc::DESCRIPTION.to_string(),
            item.loc
        ))
    );

    test_no_inheritdoc!(
        multiline_inheritdoc_violation,
        r"
        interface Test {
            /**
             * @inheritdoc Base
             */
            struct TestStruct {
                uint256 a;
            }
        }
        ",
        |item: &StructDefinition| Some(Violation::new(
            NoInheritdoc::NAME,
            NoInheritdoc::DESCRIPTION.to_string(),
            item.loc
        ))
    );
}
