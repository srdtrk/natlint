//! This module defines the rules for error items in the natlint linter.

mod missing_notice;
mod missing_param;
mod no_author;
mod no_inheritdoc;
mod no_return;
mod no_title;
mod too_many_notice;

// The rules are exported here:
pub use missing_notice::MissingNotice;
pub use missing_param::MissingParam;
pub use no_author::NoAuthor;
pub use no_inheritdoc::NoInheritdoc;
pub use no_return::NoReturn;
pub use no_title::NoTitle;
pub use too_many_notice::TooManyNotice;

/// Configuration for error rules in the linter.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
#[allow(missing_docs, clippy::struct_excessive_bools)]
pub struct ErrorRulesConfig {
    #[serde(default = "default_true")]
    pub missing_notice: bool,
    #[serde(default = "default_true")]
    pub missing_param: bool,
    #[serde(default = "default_true")]
    pub no_author: bool,
    #[serde(default = "default_true")]
    pub no_inheritdoc: bool,
    #[serde(default = "default_true")]
    pub no_return: bool,
    #[serde(default = "default_true")]
    pub no_title: bool,
    #[serde(default = "default_true")]
    pub too_many_notice: bool,
}

const fn default_true() -> bool {
    true
}

impl ErrorRulesConfig {
    /// Returns an iterator over the rules in the configuration.
    #[must_use]
    pub fn rule_set(&self) -> Vec<Box<dyn super::DynRule>> {
        let mut rules: Vec<Box<dyn super::DynRule>> = vec![];

        if self.missing_notice {
            rules.push(Box::new(MissingNotice));
        }
        if self.missing_param {
            rules.push(Box::new(MissingParam));
        }
        if self.no_author {
            rules.push(Box::new(NoAuthor));
        }
        if self.no_inheritdoc {
            rules.push(Box::new(NoInheritdoc));
        }
        if self.no_return {
            rules.push(Box::new(NoReturn));
        }
        if self.no_title {
            rules.push(Box::new(NoTitle));
        }
        if self.too_many_notice {
            rules.push(Box::new(TooManyNotice));
        }

        rules
    }
}
