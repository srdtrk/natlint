//! This module contains the supported natspec tags.

use std::str::FromStr;

use super::errors::NatspecParseError;

/// A natspec entry.
#[derive(derive_more::Display, Debug, PartialEq)]
pub enum NatspecEntry {
    /// A title that should describe the contract/interface
    #[display(fmt = "@title {_0}")]
    Title(String),
    /// The name of the author
    #[display(fmt = "@author {_0}")]
    Author(String),
    /// Explain to an end user what this does
    #[display(fmt = "@notice {_0}")]
    Notice(String),
    /// Explain to a developer any extra details
    #[display(fmt = "@dev {_0}")]
    Dev(String),
    /// Documents a parameter just like in Doxygen (must be followed by parameter name)
    #[display(fmt = "@param {_0} {_1}")]
    Param(String, String),
    /// Documents the return variables of a contract’s function
    #[display(fmt = "@return {_0} {_1}")]
    Return(String, String),
    /// Copies all missing tags from the base function (must be followed by the contract name)
    #[display(fmt = "@inheritdoc {_0}")]
    Inheritdoc(String),
    /// Custom tag, semantics is application-defined
    #[display(fmt = "@custom:{_0} {_1}")]
    Custom(String, String),
}

impl FromStr for NatspecEntry {
    type Err = NatspecParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.chars().next() != Some('@') {
            return Err(NatspecParseError::MissingNatspecTag);
        }
        let mut parts = s.splitn(2, ' ');
        let tag = parts.next().ok_or(NatspecParseError::MissingNatspecTag)?;
        let rest = parts.next().ok_or(NatspecParseError::MissingNatspecDesc)?;

        match tag {
            "@title" => Ok(NatspecEntry::Title(rest.to_string())),
            "@author" => Ok(NatspecEntry::Author(rest.to_string())),
            "@notice" => Ok(NatspecEntry::Notice(rest.to_string())),
            "@dev" => Ok(NatspecEntry::Dev(rest.to_string())),
            "@param" => {
                let mut parts = rest.splitn(2, ' ');
                let name = parts
                    .next()
                    .ok_or(NatspecParseError::MissingParameterName)?;
                let desc = parts
                    .next()
                    .ok_or(NatspecParseError::MissingParameterDesc)?;
                Ok(NatspecEntry::Param(name.to_string(), desc.to_string()))
            }
            "@return" => {
                let mut parts = rest.splitn(2, ' ');
                let name = parts
                    .next()
                    .ok_or(NatspecParseError::MissingReturnVarName)?;
                let desc = parts
                    .next()
                    .ok_or(NatspecParseError::MissingReturnVarDesc)?;
                Ok(NatspecEntry::Return(name.to_string(), desc.to_string()))
            }
            "@inheritdoc" => Ok(NatspecEntry::Inheritdoc(rest.to_string())),
            custom if custom.starts_with("@custom:") => {
                let mut parts = custom.splitn(2, ':');
                let name = parts.nth(1).ok_or(NatspecParseError::MissingCustomTag)?;
                Ok(NatspecEntry::Custom(name.to_string(), rest.to_string()))
            }
            _ => Err(NatspecParseError::UnknownTag(tag.to_string())),
        }
    }
}
