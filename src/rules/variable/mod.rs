//! This module defines the rules for variable items in the natlint linter.

mod missing_inheritdoc;
mod missing_notice;
mod no_author;
mod no_param;
mod no_return;
mod no_title;
mod too_many_inheritdoc;
mod too_many_notice;

// The rules are exported here:
pub use missing_inheritdoc::MissingInheritdoc;
pub use missing_notice::MissingNotice;
pub use no_author::NoAuthor;
pub use no_param::NoParam;
pub use no_return::NoReturn;
pub use no_title::NoTitle;
pub use too_many_inheritdoc::TooManyInheritdoc;
pub use too_many_notice::TooManyNotice;

/// Configuration for variable rules in the linter.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(missing_docs, clippy::struct_excessive_bools)]
pub struct VariableRulesConfig {
    #[serde(default = "VariableRulesConfig::default_missing_inheritdoc")]
    pub missing_inheritdoc: bool,
    #[serde(default = "VariableRulesConfig::default_missing_notice")]
    pub missing_notice: bool,
    #[serde(default = "VariableRulesConfig::default_no_author")]
    pub no_author: bool,
    #[serde(default = "VariableRulesConfig::default_no_param")]
    pub no_param: bool,
    #[serde(default = "VariableRulesConfig::default_no_return")]
    pub no_return: bool,
    #[serde(default = "VariableRulesConfig::default_no_title")]
    pub no_title: bool,
    #[serde(default = "VariableRulesConfig::default_too_many_inheritdoc")]
    pub too_many_inheritdoc: bool,
    #[serde(default = "VariableRulesConfig::default_too_many_notice")]
    pub too_many_notice: bool,
}

impl Default for VariableRulesConfig {
    fn default() -> Self {
        Self {
            missing_inheritdoc: true,
            missing_notice: true,
            no_author: true,
            no_param: true,
            no_return: true,
            no_title: true,
            too_many_inheritdoc: true,
            too_many_notice: true,
        }
    }
}

impl VariableRulesConfig {
    /// Returns an iterator over the rules in the configuration.
    #[must_use]
    pub fn rule_set(&self) -> Vec<Box<dyn super::DynRule>> {
        let mut rules: Vec<Box<dyn super::DynRule>> = vec![];

        if self.missing_inheritdoc {
            rules.push(Box::new(MissingInheritdoc));
        }
        if self.missing_notice {
            rules.push(Box::new(MissingNotice));
        }
        if self.no_author {
            rules.push(Box::new(NoAuthor));
        }
        if self.no_param {
            rules.push(Box::new(NoParam));
        }
        if self.no_return {
            rules.push(Box::new(NoReturn));
        }
        if self.no_title {
            rules.push(Box::new(NoTitle));
        }
        if self.too_many_inheritdoc {
            rules.push(Box::new(TooManyInheritdoc));
        }
        if self.too_many_notice {
            rules.push(Box::new(TooManyNotice));
        }

        rules
    }

    fn default_missing_inheritdoc() -> bool {
        Self::default().missing_inheritdoc
    }
    fn default_missing_notice() -> bool {
        Self::default().missing_notice
    }
    fn default_no_author() -> bool {
        Self::default().no_author
    }
    fn default_no_param() -> bool {
        Self::default().no_param
    }
    fn default_no_return() -> bool {
        Self::default().no_return
    }
    fn default_no_title() -> bool {
        Self::default().no_title
    }
    fn default_too_many_inheritdoc() -> bool {
        Self::default().too_many_inheritdoc
    }
    fn default_too_many_notice() -> bool {
        Self::default().too_many_notice
    }
}
