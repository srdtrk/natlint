//! This module defines the rules for event items in the natlint linter.

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

/// Configuration for event rules in the linter.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(missing_docs, clippy::struct_excessive_bools)]
pub struct EventRulesConfig {
    #[serde(default = "EventRulesConfig::default_missing_notice")]
    pub missing_notice: bool,
    #[serde(default = "EventRulesConfig::default_missing_param")]
    pub missing_param: bool,
    #[serde(default = "EventRulesConfig::default_no_author")]
    pub no_author: bool,
    #[serde(default = "EventRulesConfig::default_no_inheritdoc")]
    pub no_inheritdoc: bool,
    #[serde(default = "EventRulesConfig::default_no_return")]
    pub no_return: bool,
    #[serde(default = "EventRulesConfig::default_no_title")]
    pub no_title: bool,
    #[serde(default = "EventRulesConfig::default_too_many_notice")]
    pub too_many_notice: bool,
}

impl Default for EventRulesConfig {
    fn default() -> Self {
        Self {
            missing_notice: true,
            missing_param: true,
            no_author: true,
            no_inheritdoc: true,
            no_return: true,
            no_title: true,
            too_many_notice: true,
        }
    }
}

impl EventRulesConfig {
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

    fn default_missing_notice() -> bool {
        Self::default().missing_notice
    }
    fn default_missing_param() -> bool {
        Self::default().missing_param
    }
    fn default_no_author() -> bool {
        Self::default().no_author
    }
    fn default_no_inheritdoc() -> bool {
        Self::default().no_inheritdoc
    }
    fn default_no_return() -> bool {
        Self::default().no_return
    }
    fn default_no_title() -> bool {
        Self::default().no_title
    }
    fn default_too_many_notice() -> bool {
        Self::default().too_many_notice
    }
}
