//! Visitor pattern for traversing Solidity AST nodes.
//!
//! This module provides a simple visitor pattern implementation for solang-parser AST nodes.

use solang_parser::pt::{
    ContractPart, EnumDefinition, ErrorDefinition, EventDefinition, FunctionDefinition, SourceUnit,
    SourceUnitPart, StructDefinition, TypeDefinition, VariableDefinition,
};

/// A trait for visiting Solidity AST nodes.
pub trait Visitor {
    /// The error type returned by visitor methods.
    type Error;

    /// Visit a source unit (the root of a Solidity file).
    fn visit_source_unit(&mut self, source_unit: &mut SourceUnit) -> Result<(), Self::Error>;

    /// Visit an enum definition.
    fn visit_enum(&mut self, _enumerable: &mut EnumDefinition) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a variable definition.
    fn visit_var_definition(&mut self, _var: &mut VariableDefinition) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a function definition.
    fn visit_function(&mut self, _func: &mut FunctionDefinition) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a struct definition.
    fn visit_struct(&mut self, _structure: &mut StructDefinition) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit an event definition.
    fn visit_event(&mut self, _event: &mut EventDefinition) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit an error definition.
    fn visit_error(&mut self, _error: &mut ErrorDefinition) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a type definition.
    fn visit_type_definition(&mut self, _def: &mut TypeDefinition) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A trait for AST nodes that can be visited.
pub trait Visitable {
    /// Visit this node with the given visitor.
    fn visit<V: Visitor>(&mut self, visitor: &mut V) -> Result<(), V::Error>;
}

impl Visitable for SourceUnit {
    fn visit<V: Visitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.visit_source_unit(self)
    }
}

impl Visitable for SourceUnitPart {
    fn visit<V: Visitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Self::FunctionDefinition(func) => visitor.visit_function(func),
            Self::EventDefinition(event) => visitor.visit_event(event),
            Self::ErrorDefinition(error) => visitor.visit_error(error),
            Self::StructDefinition(structure) => visitor.visit_struct(structure),
            Self::EnumDefinition(enumerable) => visitor.visit_enum(enumerable),
            Self::VariableDefinition(var) => visitor.visit_var_definition(var),
            Self::TypeDefinition(ty) => visitor.visit_type_definition(ty),
            _ => Ok(()),
        }
    }
}

impl Visitable for ContractPart {
    fn visit<V: Visitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Self::FunctionDefinition(func) => visitor.visit_function(func),
            Self::EventDefinition(event) => visitor.visit_event(event),
            Self::ErrorDefinition(error) => visitor.visit_error(error),
            Self::StructDefinition(structure) => visitor.visit_struct(structure),
            Self::EnumDefinition(enumerable) => visitor.visit_enum(enumerable),
            Self::VariableDefinition(var) => visitor.visit_var_definition(var),
            Self::TypeDefinition(ty) => visitor.visit_type_definition(ty),
            _ => Ok(()),
        }
    }
}
