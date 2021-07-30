use crate::parser;
use crate::processors::{type_processors, value_processor};
use crate::syntax::{definers, types, variable};
use ellie_core::{defs, error, utils};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_function_caller(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::FunctionCall(ref mut functioncalldata) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                rtype: "functionCall".to_string(),
            });
        }

        if !functioncalldata.name_collected {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n")
                    && !functioncalldata.data.name.is_empty())
            {
                functioncalldata.data.name_pos.range_end = parser.pos;
                functioncalldata.data.name += letter_char;
            } else if letter_char == "(" {
                if functioncalldata.data.name.is_empty() {
                    errors.push(error::Error {
                        scope: "function_call_processor".to_string(),
                        debug_message: "d24657c187d65f0e563a4d2a66f58576".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else {
                    functioncalldata.name_collected = true;
                }
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: "function_call_processor".to_string(),
                    debug_message: "ce2f5162151608b3c494d73b3bad3913".to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if !functioncalldata.complete {
            let last_entry = functioncalldata.data.params.clone().len();
            let is_s_n = last_entry == 0
                || functioncalldata.data.params[last_entry - 1]
                    .value
                    .is_type_complete();

            if letter_char == "," && is_s_n && last_entry != 0 {
                if functioncalldata.complete {
                    errors.push(error::Error {
                        scope: "function_call_processor".to_string(),
                        debug_message: "2a7c420ef077299c074bf4358bb20bda".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else if functioncalldata.comma {
                    errors.push(error::Error {
                        scope: "function_call_processor".to_string(),
                        debug_message: "71ede01391cd28caaef4a47dc17bc462".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else {
                    if last_entry != 0 {
                        functioncalldata.data.params[last_entry - 1]
                            .value
                            .make_complete();
                    }
                    functioncalldata.comma = true;
                    functioncalldata
                        .data
                        .params
                        .push(types::function_call::FunctionCallParameter::default());
                }
            } else if letter_char == ")" && is_s_n {
                if last_entry != 0 {
                    functioncalldata.data.params[last_entry - 1].pos.range_end = parser.pos;
                }

                let fn_exists = parser.resolve_function_call(functioncalldata.clone());
                if let Some(type_errors) = fn_exists {
                    for error in type_errors {
                        errors.push(error);
                    }
                }
                functioncalldata.complete = true;
            } else {
                if letter_char != " " {
                    //TODO IS THIS SAFE ?
                    functioncalldata.comma = false;
                }

                //TODO FIX THIS with function after resolving complete
                let mut will_be_itered: variable::VariableCollector;
                if let definers::DefinerCollecting::Cloak(cloak_data) =
                    itered_data.data.rtype.clone()
                {
                    will_be_itered = if functioncalldata.data.params.is_empty() {
                        variable::VariableCollector {
                            data: variable::Variable {
                                rtype: cloak_data.rtype[0].clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    } else {
                        variable::VariableCollector {
                            data: variable::Variable {
                                value: functioncalldata.data.params
                                    [functioncalldata.data.params.len() - 1]
                                    .value
                                    .clone(),
                                rtype: cloak_data.rtype[functioncalldata.data.params.len() - 1]
                                    .clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    };
                } else {
                    will_be_itered = if functioncalldata.data.params.is_empty() {
                        variable::VariableCollector::default()
                    } else {
                        variable::VariableCollector {
                            data: variable::Variable {
                                value: functioncalldata.data.params
                                    [functioncalldata.data.params.len() - 1]
                                    .value
                                    .clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    };
                }

                let itered_fcall_vector = Box::new(value_processor::collect_value(
                    parser.clone(),
                    &mut will_be_itered,
                    letter_char,
                    next_char,
                    last_char,
                ));

                let itered_entry = match itered_fcall_vector.itered_data.data.value {
                    types::Types::Integer(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Integer(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Float(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Float(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Operator(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Operator(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Bool(match_data) => types::function_call::FunctionCallParameter {
                        value: types::Types::Bool(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            functioncalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::String(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::String(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Char(match_data) => types::function_call::FunctionCallParameter {
                        value: types::Types::Char(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            functioncalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Collective(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Collective(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Reference(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Reference(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::BraceReference(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::BraceReference(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Negative(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Negative(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Array(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Array(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Cloak(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Cloak(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::ArrowFunction(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::ArrowFunction(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::FunctionCall(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::FunctionCall(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::ClassCall(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::ClassCall(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Void => types::function_call::FunctionCallParameter {
                        value: types::Types::Void,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            functioncalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::VariableType(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::VariableType(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                functioncalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Null => types::function_call::FunctionCallParameter {
                        value: types::Types::Null,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            functioncalldata.data.params[last_entry - 1].pos
                        },
                    },
                };

                if !itered_fcall_vector.errors.is_empty() {
                    errors.extend(itered_fcall_vector.errors);
                }
                if functioncalldata.data.params.is_empty() {
                    functioncalldata.data.params.push(itered_entry);

                    if functioncalldata.data.params[0].pos.is_zero() {
                        functioncalldata.data.params[0].pos.range_start = parser.pos;
                    }
                    functioncalldata.data.params[0].pos.range_end = parser.pos;
                } else {
                    functioncalldata.data.params[last_entry - 1] = itered_entry;
                    if functioncalldata.data.params[last_entry - 1].pos.is_zero() {
                        functioncalldata.data.params[last_entry - 1].pos.range_start = parser.pos;
                    }
                    functioncalldata.data.params[last_entry - 1].pos.range_end = parser.pos;
                }
            }
        } else if letter_char == "." {
            itered_data.data.value =
                types::Types::Reference(types::reference_type::ReferenceType {
                    reference: Box::new(itered_data.data.value.clone()),
                    chain: Vec::new(),
                    on_dot: false,
                });
            type_processors::reference::collect_reference(
                parser,
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
            )
        } else if types::logical_type::LogicalOperators::is_logical_operator(letter_char)
            || types::logical_type::LogicalOperators::is_logical_operator(
                &(letter_char.to_string() + &next_char),
            )
        {
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorTypeCollector {
                    data: types::operator_type::OperatorType {
                        first: Box::new(itered_data.data.value.clone()),
                        operator: types::operator_type::Operators::LogicalType(
                            types::logical_type::LogicalOperators::Null,
                        ),
                        ..Default::default()
                    },
                    operator_collect: letter_char.to_string(),
                    first_filled: true,
                    ..Default::default()
                });
        } else if types::comparison_type::ComparisonOperators::is_comparison_operator(letter_char)
            || types::comparison_type::ComparisonOperators::is_comparison_operator(
                &(letter_char.to_string() + &next_char),
            )
        {
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorTypeCollector {
                    data: types::operator_type::OperatorType {
                        first: Box::new(itered_data.data.value.clone()),
                        operator: types::operator_type::Operators::ComparisonType(
                            types::comparison_type::ComparisonOperators::Null,
                        ),
                        ..Default::default()
                    },
                    operator_collect: letter_char.to_string(),
                    first_filled: true,
                    ..Default::default()
                });
        } else if types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(letter_char)
            || types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(
                &(letter_char.to_string() + &next_char),
            )
        {
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorTypeCollector {
                    data: types::operator_type::OperatorType {
                        first: Box::new(itered_data.data.value.clone()),
                        operator: types::operator_type::Operators::ArithmeticType(
                            types::arithmetic_type::ArithmeticOperators::Null,
                        ),
                        ..Default::default()
                    },
                    operator_collect: letter_char.to_string(),
                    first_filled: true,
                    ..Default::default()
                });
        }
    }
}
