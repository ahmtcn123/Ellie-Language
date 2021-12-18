use crate::syntax::types::variable_type;
use ellie_core::{
    defs, error,
    utils::{reliable_name_range, ReliableNameRanges},
};

impl crate::processors::Processor for variable_type::VariableTypeCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if reliable_name_range(ReliableNameRanges::VariableName, letter_char).reliable {
            if last_char == ' ' && self.data.value != "" {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "var_0x38".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            } else {
                self.complete = true;
                if self.data.value == "" {
                    self.data.pos.range_start = cursor;
                }
                self.data.pos.range_end = cursor.clone().skip_char(1);
                self.data.value += &letter_char.to_string();
            }
        } else if letter_char != ' ' {
            errors.push(error::error_list::ERROR_S1.clone().build(
                vec![error::ErrorBuildField {
                    key: "token".to_string(),
                    value: letter_char.to_string(),
                }],
                "var_0x47".to_owned(),
                defs::Cursor::build_with_skip_char(cursor),
            ));
        }
    }
}