//! This module defines the rules for function items in the natlint linter.

use solang_parser::pt::{
    ContractTy, FunctionAttribute, FunctionDefinition, FunctionTy, Visibility,
};

use crate::parser::{CommentsRef, ParseItem};

use super::super::{Rule, Violation};

/// This rule requires that all public functions have a inheritdoc comment.
pub struct RequireInheritdoc;

impl Rule<FunctionDefinition> for RequireInheritdoc {
    const NAME: &'static str = "Require Inheritdoc";
    const DESCRIPTION: &'static str =
        "All public and override functions must have an inheritdoc comment.";

    fn check(
        parent: Option<&ParseItem>,
        func: &FunctionDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // Parent must be a contract, not an interface or library
        match parent?.as_contract()?.ty {
            ContractTy::Interface(_) | ContractTy::Library(_) => return None,
            ContractTy::Contract(_) | ContractTy::Abstract(_) => (),
        };

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
        if comments.find_inheritdoc_base().is_none() {
            return Some(Violation::new(
                Self::NAME,
                Self::DESCRIPTION.to_string(),
                func.loc,
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{RequireInheritdoc, Rule};
    use crate::{
        parser::{CommentsRef, Parser},
        rules::Violation,
    };
    use forge_fmt::Visitable;
    use solang_parser::{parse, pt::FunctionDefinition};

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    /// Macro to define a test case for `RequireInheritdoc` rule
    macro_rules! test_require_inheritdoc {
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
                    RequireInheritdoc::check(Some(parent), func, comments),
                    expected
                );
            }
        };
    }

    test_require_inheritdoc!(
        public_no_violation,
        r"
        contract Test {
            /// @inheritdoc Base
            function test() public {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        external_no_violation,
        r"
        contract Test {
            /// @inheritdoc Base
            function test() external {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        override_no_violation,
        r"
        contract Test {
            /// @inheritdoc Base
            function test() override {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        private_no_violation,
        r"
        contract Test {
            function test() private {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        internal_no_violation,
        r"
        contract Test {
            function test() internal {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        modifier_no_violation,
        r"
        contract Test {
            modifier test() {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        constructor_no_violation,
        r"
        contract Test {
            constructor() {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        receive_no_violation,
        r"
        contract Test {
            receive() {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        fallback_no_violation,
        r"
        contract Test {
            fallback() {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        interface_no_violation,
        r"
        interface Test {
            function test() external;
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        library_no_violation,
        r"
        library Test {
            function test() internal;
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        public_violation,
        r"
        contract Test {
            function test() public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            RequireInheritdoc::NAME,
            RequireInheritdoc::DESCRIPTION.to_string(),
            func.loc
        ))
    );

    test_require_inheritdoc!(
        external_violation,
        r"
        contract Test {
            function test() external {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            RequireInheritdoc::NAME,
            RequireInheritdoc::DESCRIPTION.to_string(),
            func.loc
        ))
    );

    test_require_inheritdoc!(
        override_violation,
        r"
        contract Test {
            function test() override {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            RequireInheritdoc::NAME,
            RequireInheritdoc::DESCRIPTION.to_string(),
            func.loc
        ))
    );

    test_require_inheritdoc!(
        abstract_public_violation,
        r"
        abstract contract Test {
            function test() public;
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            RequireInheritdoc::NAME,
            RequireInheritdoc::DESCRIPTION.to_string(),
            func.loc
        ))
    );
}
