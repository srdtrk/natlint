use solang_parser::pt::{
    ContractTy, FunctionAttribute, FunctionDefinition, FunctionTy, Visibility,
};

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that all public functions have a inheritdoc comment.
pub struct MissingInheritdoc;

impl Rule<FunctionDefinition> for MissingInheritdoc {
    const NAME: &'static str = "MissingInheritdoc";
    const DESCRIPTION: &'static str =
        "Public and override functions must have an inheritdoc comment.";

    fn check(
        parent: Option<&ParseItem>,
        func: &FunctionDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // Parent must be a contract, not an interface or library
        match parent?.as_contract()?.ty {
            ContractTy::Interface(_) | ContractTy::Library(_) => return None,
            ContractTy::Contract(_) | ContractTy::Abstract(_) => (),
        }

        // Function must not be a modifier or constructor
        match func.ty {
            FunctionTy::Function => (),
            FunctionTy::Receive
            | FunctionTy::Fallback
            | FunctionTy::Constructor
            | FunctionTy::Modifier => return None,
        }

        // Function must be public, external, or an override
        func.attributes.iter().find(|attr| match attr {
            FunctionAttribute::Visibility(Visibility::Public(_) | Visibility::External(_))
            | FunctionAttribute::Override(..) => true,
            FunctionAttribute::Visibility(Visibility::Private(_) | Visibility::Internal(_))
            | FunctionAttribute::Mutability(_)
            | FunctionAttribute::Virtual(_)
            | FunctionAttribute::Immutable(_)
            | FunctionAttribute::BaseOrModifier(..)
            | FunctionAttribute::Error(_) => false,
        })?;

        // Function must have an inheritdoc comment
        if comments.include_tag(CommentTag::Inheritdoc).is_empty() {
            return Some(Violation::new(
                Self::NAME,
                Self::DESCRIPTION,
                ViolationError::MissingComment(CommentTag::Inheritdoc),
                func.loc,
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, FunctionDefinition, MissingInheritdoc, Rule, Violation,
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
                let func = child.as_function().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(func);

                assert_eq!(
                    MissingInheritdoc::check(Some(parent), func, comments),
                    expected
                );
            }
        };
    }

    mod public_test {
        use super::*;

        generate_missing_comment_test_cases!(
            Inheritdoc,
            test_missinginheritdoc,
            MissingInheritdoc,
            r"
                function test() public {}
            ",
            "@inheritdoc",
            FunctionDefinition
        );
    }

    mod external_test {
        use super::*;

        generate_missing_comment_test_cases!(
            Inheritdoc,
            test_missinginheritdoc,
            MissingInheritdoc,
            r"
                function test() external {}
            ",
            "@inheritdoc",
            FunctionDefinition
        );
    }

    mod override_test {
        use super::*;

        generate_missing_comment_test_cases!(
            Inheritdoc,
            test_missinginheritdoc,
            MissingInheritdoc,
            r"
                function test() override {}
            ",
            "@inheritdoc",
            FunctionDefinition
        );
    }

    test_missinginheritdoc!(
        private_no_violation,
        r"
        contract Test {
            function test() private {}
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        internal_no_violation,
        r"
        contract Test {
            function test() internal {}
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        modifier_no_violation,
        r"
        contract Test {
            modifier test() {}
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        constructor_no_violation,
        r"
        contract Test {
            constructor() {}
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        receive_no_violation,
        r"
        contract Test {
            receive() {}
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        fallback_no_violation,
        r"
        contract Test {
            fallback() {}
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        interface_no_violation,
        r"
        interface Test {
            function test() external;
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        library_no_violation,
        r"
        library Test {
            function test() internal;
        }
        ",
        |_| None
    );

    test_missinginheritdoc!(
        abstract_public_violation,
        r"
        abstract contract Test {
            function test() public;
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingInheritdoc::NAME,
            MissingInheritdoc::DESCRIPTION,
            ViolationError::MissingComment(CommentTag::Inheritdoc),
            func.loc
        ))
    );
}
