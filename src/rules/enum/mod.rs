//! This module defines the rules for enum items in the natlint linter.

use serde::{Deserialize, Serialize};

mod missing_author;
mod missing_notice;
mod missing_title;
mod missing_variant;
mod no_inheritdoc;
mod no_param;
mod no_return;
mod too_many_notice;
mod too_many_title;

// The rules are exported here:
pub use missing_author::MissingAuthor;
pub use missing_notice::MissingNotice;
pub use missing_title::MissingTitle;
pub use missing_variant::MissingVariant;
pub use no_inheritdoc::NoInheritdoc;
pub use no_param::NoParam;
pub use no_return::NoReturn;
pub use too_many_notice::TooManyNotice;
pub use too_many_title::TooManyTitle;

use crate::define_rules_config;

define_rules_config! {
    /// Configuration that contains all the rules for enum items.
    #[derive(Serialize, Deserialize)]
    #[allow(missing_docs)]
    pub struct EnumRulesConfig {
        #[serde(with = "crate::rules::r#enum::missing_author::serde_logic")]
        #[serde(default)]
        pub missing_author: Option<MissingAuthor>,

        #[serde(with = "crate::rules::r#enum::missing_notice::serde_logic")]
        #[serde(default)]
        pub missing_notice: Option<MissingNotice>,

        #[serde(with = "crate::rules::r#enum::missing_variant::serde_logic")]
        #[serde(default)]
        pub missing_variant: Option<MissingVariant>,

        #[serde(with = "crate::rules::r#enum::missing_title::serde_logic")]
        #[serde(default)]
        pub missing_title: Option<MissingTitle>,

        #[serde(with = "crate::rules::r#enum::no_inheritdoc::serde_logic")]
        #[serde(default)]
        pub no_inheritdoc: Option<NoInheritdoc>,

        #[serde(with = "crate::rules::r#enum::no_param::serde_logic")]
        #[serde(default)]
        pub no_param: Option<NoParam>,

        #[serde(with = "crate::rules::r#enum::no_return::serde_logic")]
        #[serde(default)]
        pub no_return: Option<NoReturn>,

        #[serde(with = "crate::rules::r#enum::too_many_notice::serde_logic")]
        #[serde(default)]
        pub too_many_notice: Option<TooManyNotice>,

        #[serde(with = "crate::rules::r#enum::too_many_title::serde_logic")]
        #[serde(default)]
        pub too_many_title: Option<TooManyTitle>,
    }
}
