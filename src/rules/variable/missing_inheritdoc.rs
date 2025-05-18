use solang_parser::pt::{ContractTy, VariableAttribute, VariableDefinition, Visibility};

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that all public variables have a inheritdoc comment.
pub struct MissingInheritdoc;

impl Rule for MissingInheritdoc {
    type Target = VariableDefinition;
    const NAME: &'static str = "MissingInheritdoc";
    const DESCRIPTION: &'static str =
        "Public and override variables must have an inheritdoc comment.";

    fn check(
        parent: Option<&ParseItem>,
        var: &VariableDefinition,
        comments: &CommentsRef,
    ) -> Option<Violation> {
        // Parent must be a contract, not an interface or library
        match parent?.as_contract()?.ty {
            ContractTy::Interface(_) | ContractTy::Library(_) => return None,
            ContractTy::Contract(_) | ContractTy::Abstract(_) => (),
        }

        // Variable must be public, external, or an override
        var.attrs.iter().find(|attr| match attr {
            VariableAttribute::Visibility(Visibility::Public(_) | Visibility::External(_))
            | VariableAttribute::Override(..) => true,
            VariableAttribute::Visibility(Visibility::Private(_) | Visibility::Internal(_))
            | VariableAttribute::Immutable(_)
            | VariableAttribute::Constant(_) => false,
        })?;

        // Variable must have an inheritdoc comment
        if comments.include_tag(CommentTag::Inheritdoc).is_empty() {
            return Some(Violation::new(
                Self::NAME,
                Self::DESCRIPTION,
                ViolationError::MissingComment(CommentTag::Inheritdoc),
                var.loc,
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, MissingInheritdoc, Rule, VariableDefinition, Violation,
        ViolationError,
    };
    use crate::{generate_missing_comment_test_cases, parser::Parser};
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    macro_rules! test_missinginheritdoc {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let var = child.as_variable().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(var);

                assert_eq!(
                    MissingInheritdoc::check(Some(parent), var, &comments),
                    expected
                );
            }
        };
    }

    mod pub_const_test {
        use super::*;

        generate_missing_comment_test_cases!(
            Inheritdoc,
            test_missinginheritdoc,
            MissingInheritdoc,
            r#"
                bytes32 public constant SOME_CONST = keccak256("SOME_CONST");
            "#,
            "@inheritdoc",
            VariableDefinition
        );
    }

    mod pub_immutable_test {
        use super::*;

        generate_missing_comment_test_cases!(
            Inheritdoc,
            test_missinginheritdoc,
            MissingInheritdoc,
            r"
                bytes32 public immutable SOME_IMMUT;
            ",
            "@inheritdoc",
            VariableDefinition
        );
    }

    mod pub_state_test {
        use super::*;

        generate_missing_comment_test_cases!(
            Inheritdoc,
            test_missinginheritdoc,
            MissingInheritdoc,
            r"
                State public state;
            ",
            "@inheritdoc",
            VariableDefinition
        );
    }

    test_missinginheritdoc!(
        private_no_violation,
        r"
        contract Test {
            State private state;
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        internal_no_violation,
        r"
        contract Test {
            State internal state;
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        interface_no_violation,
        r"
        interface Test {
            State public state;
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        library_no_violation,
        r"
        library Test {
            State public state;
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        abstract_public_violation,
        r"
        abstract contract Test {
            State public state;
        }
        ",
        |func: &VariableDefinition| Some(Violation::new(
            MissingInheritdoc::NAME,
            MissingInheritdoc::DESCRIPTION,
            ViolationError::MissingComment(CommentTag::Inheritdoc),
            func.loc
        ))
    );
}
