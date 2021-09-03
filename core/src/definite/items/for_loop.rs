use crate::alloc::boxed::Box;
use crate::definite::types;
use crate::defs;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ForLoop {
    pub parameter: Box<types::Types>,
    pub parameter_pos: defs::Cursor,
    pub code: String,
    pub pos: defs::Cursor,
}
