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
use serde::{Deserialize, Serialize};
pub use too_many_notice::TooManyNotice;
pub use too_many_title::TooManyTitle;

use crate::define_rules_config;

define_rules_config! {
    /// Configuration that contains all the rules for contract items.
    #[derive(Serialize, Deserialize)]
    #[allow(missing_docs)]
    pub struct ContractRulesConfig {
        #[serde(with = "crate::rules::contract::missing_author::serde_logic")]
        #[serde(default)]
        pub missing_author: Option<MissingAuthor>,

        #[serde(with = "crate::rules::contract::missing_notice::serde_logic")]
        #[serde(default)]
        pub missing_notice: Option<MissingNotice>,

        #[serde(with = "crate::rules::contract::missing_title::serde_logic")]
        #[serde(default)]
        pub missing_title: Option<MissingTitle>,

        #[serde(with = "crate::rules::contract::no_inheritdoc::serde_logic")]
        #[serde(default)]
        pub no_inheritdoc: Option<NoInheritdoc>,

        #[serde(with = "crate::rules::contract::no_param::serde_logic")]
        #[serde(default)]
        pub no_param: Option<NoParam>,

        #[serde(with = "crate::rules::contract::no_return::serde_logic")]
        #[serde(default)]
        pub no_return: Option<NoReturn>,

        #[serde(with = "crate::rules::contract::too_many_notice::serde_logic")]
        #[serde(default)]
        pub too_many_notice: Option<TooManyNotice>,

        #[serde(with = "crate::rules::contract::too_many_title::serde_logic")]
        #[serde(default)]
        pub too_many_title: Option<TooManyTitle>,
    }
}
