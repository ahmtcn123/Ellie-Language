use crate::error;
use crate::mapper;
use crate::processors;
use crate::syntax::{condition, types};

pub fn collect(
    mapper: &mut mapper::Mapper,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let mapper::Collecting::Condition(ref mut data) = mapper.current {
        if data.might_be_else_if {
            println!("Mıgt be else if");
            if data.else_if_keyword_collector == "else if" {
                data.initialized = true;
                data.cloak_collected = false;
                data.cloak_pos.range_start.0 = mapper.pos.0; //Function naming started so we set the position
                data.keyword_pos.range_start.0 = mapper.pos.0 - 1; //Function naming started so we set the position
                data.keyword_pos.range_end.0 = mapper.pos.0; //Function naming started so we set the position
                data.chains.push(condition::ConditionChain {
                    r#type: condition::ConditionType::ElseIf,
                    ..Default::default()
                });
                data.else_if_keyword_collector = "".to_string();
                data.might_be_else_if = false;
            } else if data.else_if_keyword_collector.trim() == "else" && letter_char == "{" {
                println!("Else");
                data.cloak_collected = true;
                data.initialized = true;
                data.chains.push(condition::ConditionChain {
                    r#type: condition::ConditionType::Else,
                    ..Default::default()
                });
                data.else_if_keyword_collector = "".to_string();
                data.might_be_else_if = false;
            } else {
                data.else_if_keyword_collector += letter_char;
            }
        } else if !data.initialized {
            if last_char == "i" && letter_char == "f" {
                data.initialized = true;
                data.cloak_pos.range_start.0 = mapper.pos.0; //Function naming started so we set the position
                data.keyword_pos.range_start.0 = mapper.pos.0 - 1; //Function naming started so we set the position
                data.keyword_pos.range_end.0 = mapper.pos.0; //Function naming started so we set the position
            }
        } else if !data.cloak_collected {
            if data.cloak_itered_data.data.value.is_complete() && letter_char == "{" {
                data.cloak_collected = true;
                let chain_length = if data.chains.len() == 0 {
                    0
                } else {
                    data.chains.len() - 1
                };

                if chain_length == 0 {
                    data.chains.push(condition::ConditionChain::default());
                }

                data.chains[chain_length].condition.collective.push(
                    types::cloak_type::CloakEntry {
                        value: Box::new(data.cloak_itered_data.data.value.clone()),
                        value_complete: true,
                    },
                );
            } else {
                let collected = processors::value_processor::collect(
                    &mut data.cloak_itered_data,
                    letter_char,
                    next_char.to_string(),
                    last_char.to_string(),
                    mapper.pos.clone(),
                );
                for i in collected.errors {
                    errors.push(i)
                }
                data.cloak_itered_data = collected.itered_data;
            }
        } else {
            if letter_char == "}" {
                println!("Complete");
                if data.inside_object_start {
                    if data.inside_object_count == 0 {
                        data.inside_object_start = true;
                    } else {
                        data.inside_object_count -= 1;
                    }
                } else {
                    let child_mapper = mapper::Mapper::new(
                        data.inside_code_string.clone(),
                        mapper::defs::MapperOptions {
                            functions: true,
                            break_on_error: false,
                            loops: true,
                            global_variables: true,
                            collectives: true,
                            variables: true,
                        },
                    );
                    mapper.pos = child_mapper.pos;
                    let mapped = child_mapper.Map();
                    for i in mapped.syntax_errors {
                        errors.push(i)
                    }
                    let chains_length = data.chains.clone().len() - 1;
                    data.chains[chains_length].inside_code = mapped.items;
                    mapper.collected.push(mapper.current.clone());
                    mapper.current = mapper::Collecting::None;
                    println!("Closed");
                }
            } else {
                data.inside_code_string += letter_char;
            }
        }
    }
}
