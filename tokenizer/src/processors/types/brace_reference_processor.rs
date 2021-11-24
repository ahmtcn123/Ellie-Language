use crate::processors::Processor;
use crate::syntax::types::brace_reference_type;
use ellie_core::{defs, error};

impl Processor for brace_reference_type::BraceReferenceTypeCollector {
    fn new() -> Self {
        brace_reference_type::BraceReferenceTypeCollector::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.brace_started {
            if letter_char == '[' {
                self.brace_started = true;
            } else if letter_char != ' ' {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "val".to_owned(),
                        value: letter_char.to_string(),
                    }],
                    "brace_refence_0x34".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else {
            if self.itered_cache.is_complete() && letter_char == ']' {
                self.complete = true;
                self.data.value = Box::new(self.itered_cache.current.clone());
            } else {
                self.itered_cache
                    .iterate(errors, cursor, last_char, letter_char);
            }
        }
    }
}
