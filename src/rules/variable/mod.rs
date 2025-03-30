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
use serde::{Deserialize, Serialize};
pub use too_many_inheritdoc::TooManyInheritdoc;
pub use too_many_notice::TooManyNotice;

use crate::define_rules_config;

define_rules_config! {
    /// Configuration that contains all the rules for variable items.
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(missing_docs)]
    pub struct VariableRulesConfig {
        #[serde(with = "crate::rules::variable::missing_inheritdoc::serde_logic")]
        pub missing_inheritdoc: Option<MissingInheritdoc>,

        #[serde(with = "crate::rules::variable::missing_notice::serde_logic")]
        pub missing_notice: Option<MissingNotice>,

        #[serde(with = "crate::rules::variable::no_author::serde_logic")]
        pub no_author: Option<NoAuthor>,

        #[serde(with = "crate::rules::variable::no_param::serde_logic")]
        pub no_param: Option<NoParam>,

        #[serde(with = "crate::rules::variable::no_return::serde_logic")]
        pub no_return: Option<NoReturn>,

        #[serde(with = "crate::rules::variable::no_title::serde_logic")]
        pub no_title: Option<NoTitle>,

        #[serde(with = "crate::rules::variable::too_many_inheritdoc::serde_logic")]
        pub too_many_inheritdoc: Option<TooManyInheritdoc>,

        #[serde(with = "crate::rules::variable::too_many_notice::serde_logic")]
        pub too_many_notice: Option<TooManyNotice>,
    }
}
