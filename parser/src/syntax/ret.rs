use crate::syntax::types;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ret {
    pub value: types::Types,
    pub keyword_pos: defs::Cursor,
    pub value_position: defs::Cursor,
    pub pos: defs::Cursor,
}
