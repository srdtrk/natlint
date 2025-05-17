//! This module defines the rules for struct items in the natlint linter.

mod missing_author;
mod missing_notice;
mod missing_params;
mod missing_title;
mod no_inheritdoc;
mod no_return;
mod too_many_notice;
mod too_many_title;

// The rules are exported here:
pub use missing_author::MissingAuthor;
pub use missing_notice::MissingNotice;
pub use missing_params::MissingParams;
pub use missing_title::MissingTitle;
pub use no_inheritdoc::NoInheritdoc;
pub use no_return::NoReturn;
pub use too_many_notice::TooManyNotice;
pub use too_many_title::TooManyTitle;

/// Configuration for struct rules in the linter.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
#[allow(missing_docs, clippy::struct_excessive_bools)]
pub struct StructRulesConfig {
    #[serde(default)]
    pub missing_author: bool,
    #[serde(default = "default_true")]
    pub missing_notice: bool,
    #[serde(default = "default_true")]
    pub missing_params: bool,
    #[serde(default)]
    pub missing_title: bool,
    #[serde(default = "default_true")]
    pub no_inheritdoc: bool,
    #[serde(default = "default_true")]
    pub no_return: bool,
    #[serde(default = "default_true")]
    pub too_many_notice: bool,
    #[serde(default = "default_true")]
    pub too_many_title: bool,
}

const fn default_true() -> bool {
    true
}

impl StructRulesConfig {
    /// Returns an iterator over the rules in the configuration.
    #[must_use]
    pub fn rule_set(&self) -> Vec<Box<dyn super::DynRule>> {
        let mut rules: Vec<Box<dyn super::DynRule>> = vec![];

        if self.missing_author {
            rules.push(Box::new(MissingAuthor));
        }
        if self.missing_notice {
            rules.push(Box::new(MissingNotice));
        }
        if self.missing_params {
            rules.push(Box::new(MissingParams));
        }
        if self.missing_title {
            rules.push(Box::new(MissingTitle));
        }
        if self.no_inheritdoc {
            rules.push(Box::new(NoInheritdoc));
        }
        if self.no_return {
            rules.push(Box::new(NoReturn));
        }
        if self.too_many_notice {
            rules.push(Box::new(TooManyNotice));
        }
        if self.too_many_title {
            rules.push(Box::new(TooManyTitle));
        }

        rules
    }
}
