//! This module contains the supported natspec tags.

use std::str::FromStr;

use super::errors::NatspecParseError;

/// A natspec entry.
#[derive(derive_more::Display, Debug, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum NatspecEntry {
    /// A title that should describe the contract/interface
    #[display("@title {_0}")]
    Title(String),
    /// The name of the author
    #[display("@author {_0}")]
    Author(String),
    /// Explain to an end user what this does
    #[display("@notice {_0}")]
    Notice(String),
    /// Explain to a developer any extra details
    #[display("@dev {_0}")]
    Dev(String),
    /// Documents a parameter just like in Doxygen (must be followed by parameter name)
    #[display("@param {_0} {_1}")]
    Param(String, String),
    /// Documents the return variables of a contract’s function
    #[display("@return {_0} {_1}")]
    Return(String, String),
    /// Copies all missing tags from the base function (must be followed by the contract name)
    #[display("@inheritdoc {_0}")]
    Inheritdoc(String),
    /// Custom tag, semantics is application-defined
    #[display("@custom:{_0} {_1}")]
    Custom(String, String),
}

impl FromStr for NatspecEntry {
    type Err = NatspecParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if !s.starts_with('@') {
            return Err(NatspecParseError::MissingNatspecTag);
        }
        let mut parts = s.splitn(2, ' ');
        let tag = parts.next().ok_or(NatspecParseError::MissingNatspecTag)?;
        let rest = parts
            .next()
            .filter(|s| !s.is_empty())
            .ok_or(NatspecParseError::MissingNatspecDesc)?;

        match tag {
            "@title" => Ok(Self::Title(rest.to_string())),
            "@author" => Ok(Self::Author(rest.to_string())),
            "@notice" => Ok(Self::Notice(rest.to_string())),
            "@dev" => Ok(Self::Dev(rest.to_string())),
            "@param" => {
                let mut parts = rest.splitn(2, ' ');
                let name = parts.next().ok_or_else(|| unreachable!())?;
                let desc = parts
                    .next()
                    .filter(|s| !s.is_empty())
                    .ok_or(NatspecParseError::MissingParameterDesc)?;
                Ok(Self::Param(name.to_string(), desc.to_string()))
            }
            "@return" => {
                let mut parts = rest.splitn(2, ' ');
                let name = parts.next().ok_or_else(|| unreachable!())?;
                let desc = parts
                    .next()
                    .filter(|s| !s.is_empty())
                    .ok_or(NatspecParseError::MissingReturnVarDesc)?;
                Ok(Self::Return(name.to_string(), desc.to_string()))
            }
            "@inheritdoc" => Ok(Self::Inheritdoc(rest.to_string())),
            custom if custom.starts_with("@custom:") => {
                let mut parts = custom.splitn(2, ':');
                let name = parts
                    .nth(1)
                    .filter(|s| !s.is_empty())
                    .ok_or(NatspecParseError::MissingCustomTag)?;
                Ok(Self::Custom(name.to_string(), rest.to_string()))
            }
            _ => Err(NatspecParseError::UnknownTag(tag.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test each case in a single test using test cases
    #[test]
    fn test_success_from_str() {
        let test_cases = [
            (
                "success: title",
                "@title HelloWorld",
                NatspecEntry::Title("HelloWorld".to_string()),
            ),
            (
                "success: author",
                "@author Alice",
                NatspecEntry::Author("Alice".to_string()),
            ),
            (
                "success: notice",
                "@notice A hello world example in Solidity.",
                NatspecEntry::Notice("A hello world example in Solidity.".to_string()),
            ),
            (
                "success: dev",
                "@dev A hello world example in Solidity.",
                NatspecEntry::Dev("A hello world example in Solidity.".to_string()),
            ),
            (
                "success: param",
                "@param name The name of the person to greet",
                NatspecEntry::Param(
                    "name".to_string(),
                    "The name of the person to greet".to_string(),
                ),
            ),
            (
                "success: trimmed param",
                " @param name The name of the person to greet ",
                NatspecEntry::Param(
                    "name".to_string(),
                    "The name of the person to greet".to_string(),
                ),
            ),
            (
                "success: return",
                "@return The string \"Hello, World!\"",
                NatspecEntry::Return("The".to_string(), "string \"Hello, World!\"".to_string()),
            ),
            (
                "success: inheritdoc",
                "@inheritdoc HelloWorld",
                NatspecEntry::Inheritdoc("HelloWorld".to_string()),
            ),
            (
                "success: custom",
                "@custom:mytag Some custom description",
                NatspecEntry::Custom("mytag".to_string(), "Some custom description".to_string()),
            ),
        ];

        for (name, input, expected) in &test_cases {
            assert_eq!(input.parse::<NatspecEntry>().unwrap(), *expected, "{name}");
        }
    }

    #[test]
    fn test_failure_from_str() {
        let test_cases = [
            (
                "failure: empty string",
                "",
                NatspecParseError::MissingNatspecTag,
            ),
            (
                "failure: no tag",
                "title HelloWorld",
                NatspecParseError::MissingNatspecTag,
            ),
            (
                "failure: missing natspec content",
                "@title",
                NatspecParseError::MissingNatspecDesc,
            ),
            (
                "failure: natspec content is empty",
                "@title ",
                NatspecParseError::MissingNatspecDesc,
            ),
            (
                "failure: missing param desc",
                "@param name",
                NatspecParseError::MissingParameterDesc,
            ),
            (
                "failure: missing return desc",
                "@return name",
                NatspecParseError::MissingReturnVarDesc,
            ),
            (
                "failure: missing custom tag",
                "@custom: test",
                NatspecParseError::MissingCustomTag,
            ),
            (
                "failure: unknown tag",
                "@unknown lol",
                NatspecParseError::UnknownTag("@unknown".to_string()),
            ),
        ];

        for (name, input, error) in &test_cases {
            assert_eq!(input.parse::<NatspecEntry>().unwrap_err(), *error, "{name}");
        }
    }
}
