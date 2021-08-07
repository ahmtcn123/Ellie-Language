pub use crate::parser;
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ImportItem {
    pub item: Box<parser::Collecting>,
    pub public: bool,
}