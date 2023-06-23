use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: u16,
    pub title: String,
    pub completed: bool,

    }
    } else {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Todo {
            pub id: u16,
            pub title: String,
           pub  completed: bool,
        }
    }
}
