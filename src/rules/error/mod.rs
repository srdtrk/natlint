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

use crate::define_rules_config;

define_rules_config! {
    /// Configuration that contains all the rules for error items.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(missing_docs)]
    pub struct ErrorRulesConfig {
        #[serde(with = "crate::rules::error::missing_notice::serde_logic")]
        #[serde(default)]
        pub missing_notice: Option<MissingNotice>,

        #[serde(with = "crate::rules::error::missing_param::serde_logic")]
        #[serde(default)]
        pub missing_param: Option<MissingParam>,

        #[serde(with = "crate::rules::error::no_author::serde_logic")]
        #[serde(default)]
        pub no_author: Option<NoAuthor>,

        #[serde(with = "crate::rules::error::no_inheritdoc::serde_logic")]
        #[serde(default)]
        pub no_inheritdoc: Option<NoInheritdoc>,

        #[serde(with = "crate::rules::error::no_return::serde_logic")]
        #[serde(default)]
        pub no_return: Option<NoReturn>,

        #[serde(with = "crate::rules::error::no_title::serde_logic")]
        #[serde(default)]
        pub no_title: Option<NoTitle>,

        #[serde(with = "crate::rules::error::too_many_notice::serde_logic")]
        #[serde(default)]
        pub too_many_notice: Option<TooManyNotice>,
    }
}
