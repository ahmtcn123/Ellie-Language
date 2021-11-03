use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error};

pub fn collect_integer<F, E>(
    parser: parser::Parser<F, E>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Sized,
{
    if let types::Types::Integer(ref mut data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                rtype: "int".to_owned(),
                hash: "ellie_int_hash".to_owned(),
            });
        }

        let is_num = letter_char.parse::<isize>().is_ok();
        if is_num || letter_char == "x" && data.raw.starts_with('0') {
            if data.raw == "0x" {
                panic!("[ParserError]: Hexadecimals are not supported yet")
            }

            if data.complete && last_char.parse::<isize>().is_err() && last_char != "x" {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "integer_processor".to_owned(),
                    debug_message: "45c9886b92bd2c6decf7af6fd63b84c4".to_owned(),
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
                //TODO GROW UP SİZES i8 -> i16 ~ u8 -> u16
                data.raw = data.raw.to_string() + letter_char;

                if let Ok(nm) = data.raw.parse::<i8>() {
                    data.data.value = types::integer_type::IntegerSize::I8(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::I8;
                } else if let Ok(nm) = data.raw.parse::<i16>() {
                    data.data.value = types::integer_type::IntegerSize::I16(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::I16;
                } else if let Ok(nm) = data.raw.parse::<i32>() {
                    data.data.value = types::integer_type::IntegerSize::I32(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::I32;
                } else if let Ok(nm) = data.raw.parse::<i64>() {
                    data.data.value = types::integer_type::IntegerSize::I64(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::I64;
                } else if let Ok(nm) = data.raw.parse::<i128>() {
                    data.data.value = types::integer_type::IntegerSize::I128(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::I128;
                } else if let Ok(nm) = data.raw.parse::<isize>() {
                    data.data.value = types::integer_type::IntegerSize::Isize(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::ISize;
                } else {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "integer_processor".to_owned(),
                        debug_message: "9548c52f650ef5f5f8e11e358610df74".to_owned(),
                        title: error::errorList::error_s16.title.clone(),
                        code: error::errorList::error_s16.code,
                        message: error::errorList::error_s16.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s16.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "val".to_owned(),
                                value: data.raw.clone(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                }
                data.complete = true;
            }
        } else if ellie_core::utils::is_extended(letter_char, next_char).is_some() {
            data.complete = true;
            match ellie_core::utils::is_extended(letter_char, next_char).unwrap() {
                ellie_core::utils::FoundExtended::Reference => {
                    data.complete = true;
                    if next_char.parse::<i8>().is_ok() {
                        //Float
                        itered_data.data.value =
                            types::Types::Float(types::float_type::FloatTypeCollector {
                                base: data.raw.clone(),
                                at_point: true,
                                ..Default::default()
                            });
                    } else {
                        itered_data.data.value = types::Types::Reference(
                            types::reference_type::ReferenceTypeCollector {
                                data: types::reference_type::ReferenceType {
                                    reference_pos: itered_data.data.value_pos,
                                    reference: Box::new(itered_data.data.value.clone()),
                                    chain: Vec::new(),
                                },
                                root_available: false,
                                on_dot: false,
                                complete: false,
                                last_entry: itered_data.data.value.clone().to_definer(),
                            },
                        );
                    }
                }
                ellie_core::utils::FoundExtended::BracketReference => {
                    itered_data.data.value = types::Types::BracketReference(
                        types::bracket_reference_type::BracketReferenceCollector {
                            complete: false,
                            data: types::bracket_reference_type::BracketReference {
                                pos: defs::Cursor {
                                    range_start: parser.pos,
                                    ..Default::default()
                                },
                                target: itered_data.data.value.clone().to_definer(),
                            },
                            ..Default::default()
                        },
                    );
                }
                ellie_core::utils::FoundExtended::LogicalOperator => {
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
                }
                ellie_core::utils::FoundExtended::ComparisonOperator => {
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
                }
                ellie_core::utils::FoundExtended::ArithmeticOperator => {
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
                ellie_core::utils::FoundExtended::AssignmentOperator => {
                    itered_data.data.value =
                        types::Types::Operator(types::operator_type::OperatorTypeCollector {
                            data: types::operator_type::OperatorType {
                                first: Box::new(itered_data.data.value.clone()),
                                operator: types::operator_type::Operators::AssignmentType(
                                    types::assignment_type::AssignmentOperators::Null,
                                ),
                                ..Default::default()
                            },
                            operator_collect: letter_char.to_string(),
                            first_filled: true,
                            ..Default::default()
                        });
                }
                ellie_core::utils::FoundExtended::FunctionCall => todo!(),
            }
        } else if letter_char == " " && !data.raw.is_empty() {
            data.complete = true;
        } else if letter_char != " " {
            errors.push(error::Error {
                path: parser.options.path.clone(),
                scope: "integer_processor".to_owned(),
                debug_message: "399e4d03d6c12060afcff2c6774139e8".to_owned(),
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
    } else {
        panic!("Unexpected parser behaviour")
    }
}
