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

use crate::define_rules_config;

define_rules_config! {
    /// Configuration that contains all the rules for function items.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(missing_docs)]
    pub struct FunctionRulesConfig {
        #[serde(with = "crate::rules::function::missing_inheritdoc::serde_logic")]
        #[serde(default)]
        pub missing_inheritdoc: Option<MissingInheritdoc>,

        #[serde(with = "crate::rules::function::missing_notice::serde_logic")]
        #[serde(default)]
        pub missing_notice: Option<MissingNotice>,

        #[serde(with = "crate::rules::function::missing_params::serde_logic")]
        #[serde(default)]
        pub missing_params: Option<MissingParams>,

        #[serde(with = "crate::rules::function::missing_return::serde_logic")]
        #[serde(default)]
        pub missing_return: Option<MissingReturn>,

        #[serde(with = "crate::rules::function::no_author::serde_logic")]
        #[serde(default)]
        pub no_author: Option<NoAuthor>,

        #[serde(with = "crate::rules::function::no_title::serde_logic")]
        #[serde(default)]
        pub no_title: Option<NoTitle>,

        #[serde(with = "crate::rules::function::only_inheritdoc::serde_logic")]
        #[serde(default)]
        pub only_inheritdoc: Option<OnlyInheritdoc>,
        #[serde(with = "crate::rules::function::too_many_inheritdoc::serde_logic")]
        #[serde(default)]
        pub too_many_inheritdoc: Option<TooManyInheritdoc>,

        #[serde(with = "crate::rules::function::too_many_notice::serde_logic")]
        #[serde(default)]
        pub too_many_notice: Option<TooManyNotice>,
    }
}
