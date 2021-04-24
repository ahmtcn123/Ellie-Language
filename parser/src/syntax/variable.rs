use serde::Serialize;
use ellie_core::defs;

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct Variable {
    pub name: String,
    pub dynamic:bool,
    pub value: crate::syntax::types::Types,
    pub pos : defs::Cursor
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct VariableCollector {
    pub initialized: bool,
    pub named: bool,
    pub typed: bool,
    pub value_complete: bool,
    pub r#type: String,
    pub raw_value: String,
    pub data: Variable
}