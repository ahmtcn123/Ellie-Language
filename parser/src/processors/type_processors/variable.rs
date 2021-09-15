use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::processors::type_processors;
use crate::syntax::{types, variable};
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error, utils};

pub fn collect_variable<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    let itered_data_clone = itered_data.clone();
    if let types::Types::VariableType(ref mut variable_data) = itered_data.data.value {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_owned(),
        );

        if !variable_data.value_complete {
            if current_reliability.reliable {
                if last_char == " " && !variable_data.data.value.is_empty() {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "variable_processor".to_owned(),
                        debug_message: "3414020dfb3829ef398304711b58e579".to_owned(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: current_reliability.found.to_string(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else {
                    if variable_data.data.value.is_empty() {
                        variable_data.data.pos.range_start = parser.pos;
                    }
                    variable_data.data.value += letter_char;
                    variable_data.data.pos.range_end = parser.pos.clone().skip_char(1);
                    if (next_char == ";" || next_char == "," || next_char == " ")
                        && !itered_data_clone.ignore_existence && variable_data.data.value != "new"
                    {
                        let found_target = parser.check_keyword(variable_data.data.value.clone(), false);

                        if !found_target.found {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: "variable_processor".to_owned(),
                                debug_message: "caa74f109f0e2db549da845829de2919".to_owned(),
                                title: error::errorList::error_s6.title.clone(),
                                code: error::errorList::error_s6.code,
                                message: error::errorList::error_s6.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s6.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: variable_data.data.value.clone(),
                                    }],
                                ),
                                pos: variable_data.data.pos,
                            });
                        } else {
                            match found_target.found_type {
                                parser::NameCheckResponseType::Variable(variable_type) => {
                                    if !itered_data_clone
                                        .clone()
                                        .data
                                        .rtype
                                        .same_as(variable_type.data.rtype.clone())
                                        && itered_data.data.rtype.raw_name_with_extensions() != ""
                                    {
                                        errors.push(error::Error {
                                            path: parser.options.path.clone(),
                                            scope: parser.scope.scope_name.clone(),
                                            debug_message: "dddb214fa867e5d14934fd40765fae75"
                                                .to_string(),
                                            title: error::errorList::error_s3.title.clone(),
                                            code: error::errorList::error_s3.code,
                                            message: error::errorList::error_s3.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s3.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "token1".to_owned(),
                                                        value: itered_data
                                                            .data
                                                            .rtype
                                                            .raw_name_with_extensions(),
                                                    },
                                                    error::ErrorBuildField {
                                                        key: "token2".to_owned(),
                                                        value: variable_type
                                                            .data
                                                            .rtype
                                                            .raw_name_with_extensions(),
                                                    },
                                                ],
                                            ),
                                            pos: variable_data.data.pos,
                                        });
                                    }
                                }
                                _ => {
                                    panic!("Inferring language items to variables are not yet supported")
                                }
                            }
                        }
                    }
                }

                if variable_data.data.value == "true" || variable_data.data.value == "false" {
                    itered_data.data.value = types::Types::Bool(types::bool_type::BoolType {
                        value: variable_data.data.value == "true",
                        raw: variable_data.data.value.clone(),
                    });
                } else if variable_data.data.value == "new" && next_char == " " {
                    itered_data.data.value = types::Types::ConstructedClass(
                        types::constructed_class::ConstructedClassCollector {
                            keyword_index: 3,
                            data: types::constructed_class::ConstructedClass {
                                value: Box::new(types::Types::Null),
                                keyword_pos: defs::Cursor {
                                    range_start: parser.pos.clone().pop_char(3),
                                    range_end: parser.pos.clone(),
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    );
                }
            } else if !variable_data.data.value.is_empty() {
                if letter_char == ";" {
                    variable_data.value_complete = true;
                } else if letter_char == "." {
                    variable_data.value_complete = true;
                    itered_data.data.value =
                        types::Types::Reference(types::reference_type::ReferenceTypeCollector {
                            data: types::reference_type::ReferenceType {
                                reference_pos: variable_data.data.pos,
                                reference: Box::new(itered_data.data.value.clone()),
                                chain: Vec::new(),
                            },
                            root_available: false,
                            on_dot: false,
                            complete: false,
                        });
                    type_processors::reference::collect_reference(
                        parser.clone(),
                        itered_data,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                } else if letter_char == "(" {
                    itered_data.data.value =
                        types::Types::FunctionCall(types::function_call::FunctionCallCollector {
                            data: types::function_call::FunctionCall {
                                name: variable_data.data.value.clone(),
                                name_pos: defs::Cursor {
                                    range_start: variable_data.data.pos.range_start,
                                    range_end: variable_data
                                        .data
                                        .pos
                                        .range_end
                                        .clone()
                                        .skip_char(1),
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    type_processors::function_call::collect_function_caller(
                        parser.clone(),
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
                    variable_data.value_complete = true;
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
                } else if types::comparison_type::ComparisonOperators::is_comparison_operator(
                    letter_char,
                ) || types::comparison_type::ComparisonOperators::is_comparison_operator(
                    &(letter_char.to_string() + &next_char),
                ) {
                    variable_data.value_complete = true;
                    itered_data.data.value =
                        types::Types::Operator(types::operator_type::OperatorTypeCollector {
                            data: types::operator_type::OperatorType {
                                first: Box::new(itered_data.data.value.clone()),
                                operator: types::operator_type::Operators::ComparisonType(
                                    types::comparison_type::ComparisonOperators::Null,
                                ),
                                ..Default::default()
                            },
                            first_filled: true,
                            operator_collect: letter_char.to_string(),
                            ..Default::default()
                        });
                } else if types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(
                    letter_char,
                ) || types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(
                    &(letter_char.to_string() + &next_char),
                ) {
                    variable_data.value_complete = true;
                    itered_data.data.value =
                        types::Types::Operator(types::operator_type::OperatorTypeCollector {
                            data: types::operator_type::OperatorType {
                                first: Box::new(itered_data.data.value.clone()),
                                operator: types::operator_type::Operators::ArithmeticType(
                                    types::arithmetic_type::ArithmeticOperators::Null,
                                ),
                                ..Default::default()
                            },
                            first_filled: true,
                            operator_collect: letter_char.to_string(),
                            ..Default::default()
                        });
                } else if letter_char != " " {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "variable_processor".to_owned(),
                        debug_message: "90a2e3d388da82b9bf97b4070fd8d7f2".to_owned(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: current_reliability.found.to_string(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                }
            } else if letter_char != " " {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "variable_processor".to_owned(),
                    debug_message: "4f5f6ca726ba5bc9076e58d0aa819b53".to_owned(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: current_reliability.found.to_string(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        }
    }
}
