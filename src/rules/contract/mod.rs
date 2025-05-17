//! This module defines the rules for contract items in the natlint linter.

mod missing_author;
mod missing_notice;
mod missing_title;
mod no_inheritdoc;
mod no_param;
mod no_return;
mod too_many_notice;
mod too_many_title;

// The rules are exported here:
pub use missing_author::MissingAuthor;
pub use missing_notice::MissingNotice;
pub use missing_title::MissingTitle;
pub use no_inheritdoc::NoInheritdoc;
pub use no_param::NoParam;
pub use no_return::NoReturn;
pub use too_many_notice::TooManyNotice;
pub use too_many_title::TooManyTitle;

/// Configuration for contract rules in the linter.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(missing_docs, clippy::struct_excessive_bools)]
pub struct ContractRulesConfig {
    #[serde(default = "ContractRulesConfig::default_missing_author")]
    pub missing_author: bool,
    #[serde(default = "ContractRulesConfig::default_missing_notice")]
    pub missing_notice: bool,
    #[serde(default = "ContractRulesConfig::default_missing_title")]
    pub missing_title: bool,
    #[serde(default = "ContractRulesConfig::default_no_inheritdoc")]
    pub no_inheritdoc: bool,
    #[serde(default = "ContractRulesConfig::default_no_param")]
    pub no_param: bool,
    #[serde(default = "ContractRulesConfig::default_no_return")]
    pub no_return: bool,
    #[serde(default = "ContractRulesConfig::default_too_many_notice")]
    pub too_many_notice: bool,
    #[serde(default = "ContractRulesConfig::default_too_many_title")]
    pub too_many_title: bool,
}

impl Default for ContractRulesConfig {
    fn default() -> Self {
        Self {
            missing_author: false,
            missing_notice: true,
            missing_title: true,
            no_inheritdoc: false,
            no_param: true,
            no_return: true,
            too_many_notice: true,
            too_many_title: true,
        }
    }
}

impl ContractRulesConfig {
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
        if self.missing_title {
            rules.push(Box::new(MissingTitle));
        }
        if self.no_inheritdoc {
            rules.push(Box::new(NoInheritdoc));
        }
        if self.no_param {
            rules.push(Box::new(NoParam));
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

    fn default_missing_author() -> bool {
        Self::default().missing_author
    }
    fn default_missing_notice() -> bool {
        Self::default().missing_notice
    }
    fn default_missing_title() -> bool {
        Self::default().missing_title
    }
    fn default_no_inheritdoc() -> bool {
        Self::default().no_inheritdoc
    }
    fn default_no_param() -> bool {
        Self::default().no_param
    }
    fn default_no_return() -> bool {
        Self::default().no_return
    }
    fn default_too_many_notice() -> bool {
        Self::default().too_many_notice
    }
    fn default_too_many_title() -> bool {
        Self::default().too_many_title
    }
}
