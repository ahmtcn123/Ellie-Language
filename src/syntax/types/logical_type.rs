use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum LogicalOpearators {
    And,
    Or,
    Null
}

impl Default for LogicalOpearators {
    fn default() -> Self {
        LogicalOpearators::Null
    }
}

impl LogicalOpearators {
    pub fn is_opearator(value: &str) -> bool {
        "|&".contains(value)
    }

    pub fn resolve_operator(value: &str) -> Result<LogicalOpearators, bool> {
        if value == "&&" {
            Ok(LogicalOpearators::And)
        } else if value == "||" {
            Ok(LogicalOpearators::Or)
        } else {
            Err(true)
        }
    }
}


#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct LogicalType {
    pub cloaked: bool,
    pub first: Box<crate::syntax::types::Types>,
    pub first_filled: bool,
    pub second: Box<crate::syntax::types::Types>,
    pub operator: LogicalOpearators,
    pub operator_collect: String,
    pub operator_collected: bool,
    pub child_start: bool
}