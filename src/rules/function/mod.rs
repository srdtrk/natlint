//! This module defines the rules for function items in the natlint linter.

mod missing_inheritdoc;
mod missing_notice;
mod missing_params;
mod missing_return;
mod no_author;
mod no_title;
mod only_inheritdoc;
mod too_many_inheritdoc;
mod too_many_notice;

// The rules are exported here:
pub use missing_inheritdoc::MissingInheritdoc;
pub use missing_notice::MissingNotice;
pub use missing_params::MissingParams;
pub use missing_return::MissingReturn;
pub use no_author::NoAuthor;
pub use no_title::NoTitle;
pub use only_inheritdoc::OnlyInheritdoc;
pub use too_many_inheritdoc::TooManyInheritdoc;
pub use too_many_notice::TooManyNotice;

/// Configuration for function rules in the linter.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(missing_docs, clippy::struct_excessive_bools)]
pub struct FunctionRulesConfig {
    #[serde(default = "FunctionRulesConfig::default_missing_inheritdoc")]
    pub missing_inheritdoc: bool,
    #[serde(default = "FunctionRulesConfig::default_missing_notice")]
    pub missing_notice: bool,
    #[serde(default = "FunctionRulesConfig::default_missing_params")]
    pub missing_params: bool,
    #[serde(default = "FunctionRulesConfig::default_missing_return")]
    pub missing_return: bool,
    #[serde(default = "FunctionRulesConfig::default_no_author")]
    pub no_author: bool,
    #[serde(default = "FunctionRulesConfig::default_no_title")]
    pub no_title: bool,
    #[serde(default = "FunctionRulesConfig::default_only_inheritdoc")]
    pub only_inheritdoc: bool,
    #[serde(default = "FunctionRulesConfig::default_too_many_inheritdoc")]
    pub too_many_inheritdoc: bool,
    #[serde(default = "FunctionRulesConfig::default_too_many_notice")]
    pub too_many_notice: bool,
}

impl Default for FunctionRulesConfig {
    fn default() -> Self {
        Self {
            missing_inheritdoc: true,
            missing_notice: true,
            missing_params: true,
            missing_return: true,
            no_author: true,
            no_title: true,
            only_inheritdoc: false,
            too_many_inheritdoc: true,
            too_many_notice: true,
        }
    }
}

impl FunctionRulesConfig {
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
        if self.missing_params {
            rules.push(Box::new(MissingParams));
        }
        if self.missing_return {
            rules.push(Box::new(MissingReturn));
        }
        if self.no_author {
            rules.push(Box::new(NoAuthor));
        }
        if self.no_title {
            rules.push(Box::new(NoTitle));
        }
        if self.only_inheritdoc {
            rules.push(Box::new(OnlyInheritdoc));
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
    fn default_missing_params() -> bool {
        Self::default().missing_params
    }
    fn default_missing_return() -> bool {
        Self::default().missing_return
    }
    fn default_no_author() -> bool {
        Self::default().no_author
    }
    fn default_no_title() -> bool {
        Self::default().no_title
    }
    fn default_only_inheritdoc() -> bool {
        Self::default().only_inheritdoc
    }
    fn default_too_many_inheritdoc() -> bool {
        Self::default().too_many_inheritdoc
    }
    fn default_too_many_notice() -> bool {
        Self::default().too_many_notice
    }
}
