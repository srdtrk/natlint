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

use crate::define_rules_config;

define_rules_config! {
    /// Configuration that contains all the rules for struct items.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(missing_docs)]
    pub struct StructRulesConfig {
        #[serde(with = "crate::rules::r#struct::missing_author::serde_logic")]
        #[serde(default)]
        pub missing_author: Option<MissingAuthor>,

        #[serde(with = "crate::rules::r#struct::missing_notice::serde_logic")]
        #[serde(default)]
        pub missing_notice: Option<MissingNotice>,

        #[serde(with = "crate::rules::r#struct::missing_params::serde_logic")]
        #[serde(default)]
        pub missing_params: Option<MissingParams>,

        #[serde(with = "crate::rules::r#struct::missing_title::serde_logic")]
        #[serde(default)]
        pub missing_title: Option<MissingTitle>,

        #[serde(with = "crate::rules::r#struct::no_inheritdoc::serde_logic")]
        #[serde(default)]
        pub no_inheritdoc: Option<NoInheritdoc>,

        #[serde(with = "crate::rules::r#struct::no_return::serde_logic")]
        #[serde(default)]
        pub no_return: Option<NoReturn>,

        #[serde(with = "crate::rules::r#struct::too_many_notice::serde_logic")]
        #[serde(default)]
        pub too_many_notice: Option<TooManyNotice>,

        #[serde(with = "crate::rules::r#struct::too_many_title::serde_logic")]
        #[serde(default)]
        pub too_many_title: Option<TooManyTitle>,
    }
}
