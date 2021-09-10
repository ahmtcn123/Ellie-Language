use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::processors::{type_processors, value_processor};
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error};

pub fn collect_cloak<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    if let types::Types::Cloak(ref mut cloak_data) = itered_data.data.value {
        let mut last_entry = cloak_data.data.clone().collective.len();

        let is_s_n = last_entry == 0
            || cloak_data.data.collective[last_entry - 1]
                .value
                .is_type_complete();

        if letter_char == "(" && !cloak_data.child_start && is_s_n {
            if !cloak_data.comma && last_entry != 0 {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "cloak_processor".to_owned(),
                    debug_message: "475b7b33af65e9dd8cc2dc7b11b83932".to_owned(),
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
                cloak_data.child_start = true;
                if last_entry == 0 {
                    cloak_data
                        .data
                        .collective
                        .push(types::cloak_type::CloakEntry {
                            value_complete: false,
                            value: Box::new(types::Types::Cloak(
                                types::cloak_type::CloakTypeCollector::default(),
                            )),
                            location: defs::Cursor {
                                range_start: parser.pos,
                                ..Default::default()
                            },
                        });
                } else {
                    cloak_data.data.collective[last_entry - 1] = types::cloak_type::CloakEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Cloak(
                            types::cloak_type::CloakTypeCollector::default(),
                        )),
                        location: defs::Cursor {
                            range_start: parser.pos,
                            ..Default::default()
                        },
                    };
                }
            }
        } else if letter_char == "," && !cloak_data.child_start && is_s_n {
            if cloak_data.complete {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "cloak_processor".to_owned(),
                    debug_message: "9a4c2d33662fbfcf01d9a193a0b8599c".to_owned(),
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
            } else if cloak_data.comma {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "cloak_processor".to_owned(),
                    debug_message: "f97a37d6420f18d795ed119b076566b4".to_owned(),
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
                    cloak_data.data.collective[last_entry - 1]
                        .value
                        .make_complete();
                    cloak_data.data.collective[last_entry - 1].value_complete = true;
                }

                if !itered_data.data.dynamic {
                    if let definers::DefinerCollecting::Cloak(cloak_defining) =
                        itered_data.data.rtype.clone()
                    {
                        let entry_type = parser.resolve_variable(
                            *cloak_data.data.collective[last_entry - 1].value.clone(),
                        );
                        if cloak_defining.rtype.len() > last_entry - 1
                            && cloak_defining.rtype[last_entry - 1].raw_name() != entry_type
                        {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "61629bcde0c1dbe08d1f789ed8c4d43f".to_owned(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_owned(),
                                            value: cloak_defining.rtype[last_entry - 1].raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_owned(),
                                            value: entry_type,
                                        },
                                    ],
                                ),
                                pos: cloak_data.data.collective[last_entry - 1].location,
                            });
                        }
                    }
                }

                cloak_data.comma = true;
                cloak_data.data.layer_size += 1;
                cloak_data
                    .data
                    .collective
                    .push(types::cloak_type::CloakEntry::default());
            }
        } else if letter_char == ")" && !cloak_data.child_start && is_s_n {
            if cloak_data.comma {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "cloak_processor".to_owned(),
                    debug_message: "4868d96122d5692c6dfb807226ce3c07".to_owned(),
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
            } else if cloak_data.complete {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "cloak_processor".to_owned(),
                    debug_message: "860f8ba1c0099cd2ef852655cedb56f8".to_owned(),
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
                    if cloak_data.data.collective[last_entry - 1].value
                        == Box::new(types::Types::Null)
                    {
                        cloak_data.data.collective.pop();
                    } else {
                        cloak_data.data.collective[last_entry - 1].value_complete = true;
                        cloak_data.data.collective[last_entry - 1]
                            .value
                            .make_complete();
                    }
                }

                if !itered_data.data.dynamic {
                    if let definers::DefinerCollecting::Cloak(cloak_defining) =
                        itered_data.data.rtype.clone()
                    {
                        let entry_type = parser.resolve_variable(
                            *cloak_data.data.collective[last_entry - 1].value.clone(),
                        );
                        if cloak_defining.rtype.len() > last_entry - 1
                            && cloak_defining.rtype[last_entry - 1].raw_name() != entry_type
                        {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "b796cc238354e7fa61b70d90728737c2".to_owned(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_owned(),
                                            value: cloak_defining.rtype[last_entry - 1].raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_owned(),
                                            value: entry_type,
                                        },
                                    ],
                                ),
                                pos: cloak_data.data.collective[last_entry - 1].location,
                            });
                        }
                    }
                }

                cloak_data.data.layer_size += 1;
                cloak_data.complete = true;
                itered_data.value_complete = true;
            }
        } else if cloak_data.complete && letter_char == "." && is_s_n {
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
                });
            type_processors::reference::collect_reference(
                parser.clone(),
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
            )
        } else if cloak_data.complete
            && is_s_n
            && types::logical_type::LogicalOperators::is_logical_operator(letter_char)
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
        } else if cloak_data.complete
            && is_s_n
            && types::comparison_type::ComparisonOperators::is_comparison_operator(letter_char)
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
        } else if cloak_data.complete
            && is_s_n
            && types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(letter_char)
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
        } else {
            if letter_char != " " {
                //TODO IS THIS SAFE ?
                cloak_data.comma = false;
            }

            let mut will_be_itered: variable::VariableCollector;
            if let definers::DefinerCollecting::Cloak(cloak_cloak_data) =
                itered_data.data.rtype.clone()
            {
                will_be_itered = if cloak_data.data.collective.is_empty() {
                    variable::VariableCollector {
                        data: variable::Variable {
                            rtype: cloak_cloak_data.rtype[0].clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                } else if cloak_cloak_data.rtype.len() == cloak_data.data.collective.len() {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *cloak_data.data.collective
                                [cloak_data.data.collective.len() - 1]
                                .value
                                .clone(),
                            rtype: cloak_cloak_data.rtype[cloak_data.data.collective.len() - 1]
                                .clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                } else {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *cloak_data.data.collective
                                [cloak_data.data.collective.len() - 1]
                                .value
                                .clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                }
            } else {
                will_be_itered = if cloak_data.data.collective.is_empty() {
                    variable::VariableCollector::default()
                } else {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *cloak_data.data.collective
                                [cloak_data.data.collective.len() - 1]
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

            if let types::Types::Cloak(ref acloak_data) = will_be_itered.data.value {
                if acloak_data.complete {
                    cloak_data.child_start = false;
                }
            }

            let itered_entry = match will_be_itered.data.value {
                types::Types::Integer(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: match_cloak_data.complete,
                    value: Box::new(types::Types::Integer(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                            && letter_char != " "
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Float(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: match_cloak_data.complete,
                    value: Box::new(types::Types::Float(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                            && letter_char != " "
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Operator(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Operator(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Bool(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Bool(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                            && letter_char != " "
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::String(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: match_cloak_data.complete,
                    value: Box::new(types::Types::String(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Char(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: match_cloak_data.complete,
                    value: Box::new(types::Types::Char(match_cloak_data.clone())),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                            && match_cloak_data.value.clone() != '\0'
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Collective(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Collective(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Reference(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Reference(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Negative(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Negative(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Array(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Array(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Cloak(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Cloak(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::ArrowFunction(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::ArrowFunction(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::FunctionCall(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::FunctionCall(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::ConstructedClass(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::ConstructedClass(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Void => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::VariableType(match_cloak_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::VariableType(match_cloak_data)),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                            && letter_char != " "
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Null => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                    location: defs::Cursor {
                        range_start: if cloak_data.data.collective.len() != 0
                            && !cloak_data.data.collective[last_entry - 1]
                                .location
                                .is_zero()
                        {
                            cloak_data.data.collective[last_entry - 1]
                                .location
                                .range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
            };

            if cloak_data.data.collective.is_empty() {
                cloak_data.data.collective.push(itered_entry);
                last_entry = 1;
            } else {
                cloak_data.data.collective[last_entry - 1] = itered_entry;
            }

            cloak_data.data.collective[last_entry - 1]
                .location
                .range_end = parser.pos.clone().skip_char(1);

            if let definers::DefinerCollecting::Cloak(cloak_def) = itered_data.data.rtype.clone() {
                if cloak_def.rtype.len() < cloak_data.data.collective.len() && letter_char != " " {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "cloak_processor".to_owned(),
                        debug_message: "39183247e141e0606b72340a6a8ff49f".to_owned(),
                        title: error::errorList::error_s19.title.clone(),
                        code: error::errorList::error_s19.code,
                        message: error::errorList::error_s19.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s19.message.clone(),
                            vec![
                                error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: cloak_def.rtype.len().to_string(),
                                },
                                error::ErrorBuildField {
                                    key: "token2".to_owned(),
                                    value: cloak_data.data.collective.len().to_string(),
                                },
                            ],
                        ),
                        pos: cloak_data.data.collective[last_entry - 1].location,
                    });
                }
            }
        }
    }
}
