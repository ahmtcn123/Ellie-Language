use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error, utils};

pub fn collect_function_caller<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    if let types::Types::FunctionCall(ref mut function_call_data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                rtype: "functionCall".to_owned(),
            });
        }

        if !function_call_data.name_collected {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n")
                    && !function_call_data.data.name.is_empty())
            {
                function_call_data.data.name_pos.range_end = parser.pos;
                function_call_data.data.name += letter_char;
            } else if letter_char == "(" {
                if function_call_data.data.name.is_empty() {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "function_call_processor".to_owned(),
                        debug_message: "3232af9bf0388ea76b2d96e9c3a105dc".to_owned(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else {
                    function_call_data.name_collected = true;
                }
            } else if letter_char != " " {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "function_call_processor".to_owned(),
                    debug_message: "81b24ba56ae1222694450f1cd1d1e8cf".to_owned(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: letter_char.to_string(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if !function_call_data.complete {
            let last_entry = function_call_data.data.params.clone().len();
            let is_s_n = last_entry == 0
                || function_call_data.data.params[last_entry - 1]
                    .value
                    .is_type_complete();

            if letter_char == "," && is_s_n && last_entry != 0 {
                if function_call_data.complete {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "function_call_processor".to_owned(),
                        debug_message: "c78852de7c43978c9fd158b24d873c17".to_owned(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else if function_call_data.comma {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "function_call_processor".to_owned(),
                        debug_message: "a35a1b4e5e6db073994c001163ba27dc".to_owned(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
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
                        function_call_data.data.params[last_entry - 1]
                            .value
                            .make_complete();
                    }
                    function_call_data.comma = true;
                    function_call_data
                        .data
                        .params
                        .push(types::function_call::FunctionCallParameter::default());
                }
            } else if letter_char == ")" && is_s_n {
                if last_entry != 0 {
                    function_call_data.data.params[last_entry - 1].pos.range_end = parser.pos;
                }

                let fn_exists = parser.resolve_function_call(function_call_data.clone());
                match fn_exists {
                    Ok(return_type) => function_call_data.return_type = return_type,
                    Err(type_errors) => {
                        for error in type_errors {
                            errors.push(error);
                        }
                    }
                }
                function_call_data.complete = true;
            } else {
                if letter_char != " " {
                    //TODO IS THIS SAFE ?
                    function_call_data.comma = false;
                }

                //TODO FIX THIS with function after resolving complete
                let mut will_be_itered: variable::VariableCollector;
                if let definers::DefinerCollecting::Cloak(cloak_data) =
                    itered_data.data.rtype.clone()
                {
                    will_be_itered = if function_call_data.data.params.is_empty() {
                        variable::VariableCollector {
                            ignore_existence: itered_data.ignore_existence,
                            data: variable::Variable {
                                rtype: cloak_data.rtype[0].clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    } else {
                        variable::VariableCollector {
                            data: variable::Variable {
                                value: function_call_data.data.params
                                    [function_call_data.data.params.len() - 1]
                                    .value
                                    .clone(),
                                rtype: cloak_data.rtype[function_call_data.data.params.len() - 1]
                                    .clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    };
                } else {
                    will_be_itered = if function_call_data.data.params.is_empty() {
                        variable::VariableCollector {
                            ignore_existence: itered_data.ignore_existence,
                            ..Default::default()
                        }
                    } else {
                        variable::VariableCollector {
                            data: variable::Variable {
                                value: function_call_data.data.params
                                    [function_call_data.data.params.len() - 1]
                                    .value
                                    .clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    };
                }

                value_processor::collect_value(
                    parser.clone(),
                    &mut will_be_itered,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                );

                let itered_entry = match will_be_itered.data.value {
                    types::Types::Integer(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Integer(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Float(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Float(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Operator(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Operator(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Bool(match_data) => types::function_call::FunctionCallParameter {
                        value: types::Types::Bool(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            function_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::String(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::String(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Char(match_data) => types::function_call::FunctionCallParameter {
                        value: types::Types::Char(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            function_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Collective(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Collective(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Reference(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Reference(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::NullResolver(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::NullResolver(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Negative(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Negative(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Array(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Array(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Cloak(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::Cloak(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::ArrowFunction(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::ArrowFunction(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::FunctionCall(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::FunctionCall(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::ConstructedClass(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::ConstructedClass(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Void => types::function_call::FunctionCallParameter {
                        value: types::Types::Void,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            function_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::VariableType(match_data) => {
                        types::function_call::FunctionCallParameter {
                            value: types::Types::VariableType(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                function_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Null => types::function_call::FunctionCallParameter {
                        value: types::Types::Null,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            function_call_data.data.params[last_entry - 1].pos
                        },
                    },
                };

                if function_call_data.data.params.is_empty() {
                    function_call_data.data.params.push(itered_entry);

                    if function_call_data.data.params[0].pos.is_zero() {
                        function_call_data.data.params[0].pos.range_start = parser.pos;
                    }
                    function_call_data.data.params[0].pos.range_end = parser.pos;
                } else {
                    function_call_data.data.params[last_entry - 1] = itered_entry;
                    if function_call_data.data.params[last_entry - 1].pos.is_zero() {
                        function_call_data.data.params[last_entry - 1]
                            .pos
                            .range_start = parser.pos;
                    }
                    function_call_data.data.params[last_entry - 1].pos.range_end = parser.pos;
                }
            }
        } else if letter_char == "." {
            itered_data.data.value =
                types::Types::Reference(types::reference_type::ReferenceTypeCollector {
                    data: types::reference_type::ReferenceType {
                        reference_pos: itered_data.data.value_pos,
                        reference: Box::new(itered_data.data.value.clone()),
                        chain: Vec::new(),
                    },
                    root_available: false,
                    on_dot: false,
                    complete: false,
                    last_entry: itered_data.data.value.clone().to_definer(),
                });
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
    } else {
        panic!("Unexpected parser behaviour")
    }
}
