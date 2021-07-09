use crate::parser;
use crate::processors;
use crate::syntax::function;
use crate::syntax::{definers, types, variable};
use ellie_core::{defs, error, utils};

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_arrow(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::ArrowFunction(ref mut functiondata) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype =
                definers::DefinerCollecting::Function(definers::FunctionType::default());
        }

        if !functiondata.parameter_wrote {
            if letter_char == "(" && !functiondata.param_bracket_opened {
                functiondata.param_bracket_opened = true;
            } else {
                let mut last_entry = functiondata.data.parameters.len();
                let typing_name = if last_entry == 0 {
                    true
                } else {
                    !functiondata.data.parameters[last_entry - 1].named
                };

                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );

                if typing_name {
                    if current_reliability.reliable
                        && ((last_char != " " && last_char != "\n")
                            || last_entry == 0
                            || functiondata.data.parameters[last_entry - 1]
                                .data
                                .name
                                .is_empty())
                    {
                        if last_entry == 0 {
                            functiondata
                                .data
                                .parameters
                                .push(function::FunctionParameterCollector::default());
                            last_entry = 1;
                        }
                        if functiondata.data.parameters[last_entry - 1]
                            .data
                            .name
                            .is_empty()
                        {
                            functiondata.data.parameters[last_entry - 1]
                                .data
                                .pos
                                .range_start = parser.pos;
                        }
                        if functiondata.data.parameters[last_entry - 1]
                            .data
                            .name_pos
                            .range_start
                            .is_zero()
                            && letter_char != " "
                        {
                            functiondata.data.parameters[last_entry - 1]
                                .data
                                .name_pos
                                .range_start = parser.pos;
                        }
                        functiondata.data.parameters[last_entry - 1]
                            .data
                            .name_pos
                            .range_end = parser.pos.clone().skipChar(1);
                        functiondata.data.parameters[last_entry - 1].data.name += letter_char
                    } else if letter_char == ":" {
                        if last_entry == 0
                            || functiondata.data.parameters[last_entry - 1]
                                .data
                                .name
                                .is_empty()
                        {
                            errors.push(error::Error {
                                scope: "arrow_function".to_string(),
                                debug_message: "2c7dccb9dbff02c405315804796d160f".to_string(),
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
                                    range_end: parser.pos.clone().skipChar(1),
                                },
                            });
                        } else {
                            functiondata.data.parameters[last_entry - 1].named = true;
                        }
                    } else if letter_char == ")" && last_entry == 0 {
                        functiondata.parameter_wrote = true;
                    } else if letter_char != " " {
                        errors.push(error::Error {
                            scope: "arrow_function".to_string(),
                            debug_message: "7b3c80ddb3d8089d9b111749147c9833".to_string(),
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
                                range_end: parser.pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if letter_char == ")"
                    && (last_entry == 0
                        || functiondata.data.parameters[last_entry - 1].child_brace == 0)
                {
                    if functiondata.has_dedup() {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "e3ed995ff6d87f0e0ea4a808be9dfa9c".to_string(),
                            title: error::errorList::error_s10.title.clone(),
                            code: error::errorList::error_s10.code,
                            message: error::errorList::error_s10.message.clone(),
                            builded_message: error::BuildedError::build_from_string(
                                error::errorList::error_s10.message.clone(),
                            ),
                            pos: functiondata.data.parameters[last_entry - 1].data.name_pos,
                        });
                    }
                    if let definers::DefinerCollecting::Function(function) =
                        itered_data.data.rtype.clone()
                    {
                        if !functiondata.data.parameters[last_entry - 1]
                            .data
                            .rtype
                            .clone()
                            .same_as(function.params[last_entry - 1].clone())
                        {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "qq".to_string(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: function.params[last_entry - 1].raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: functiondata.data.parameters[last_entry - 1]
                                                .data
                                                .rtype
                                                .raw_name(),
                                        },
                                    ],
                                ),
                                pos: functiondata.data.parameters[last_entry - 1].data.type_pos,
                            });
                        }
                    }
                    if let definers::DefinerCollecting::Generic(name) =
                        &functiondata.data.parameters[last_entry - 1].data.rtype
                    {
                        if !parser.type_exists(name.rtype.clone()) {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "replace".to_string(),
                                title: error::errorList::error_s6.title.clone(),
                                code: error::errorList::error_s6.code,
                                message: error::errorList::error_s6.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s6.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: name.rtype.clone(),
                                    }],
                                ),
                                pos: functiondata.data.parameters[last_entry - 1].data.type_pos,
                            });
                        }
                    }
                    functiondata.parameter_wrote = true;
                } else if letter_char == ","
                    && functiondata.data.parameters[last_entry - 1]
                        .data
                        .rtype
                        .is_definer_complete()
                {
                    if functiondata.has_dedup() {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "e3ed995ff6d87f0e0ea4a808be9dfa9c".to_string(),
                            title: error::errorList::error_s10.title.clone(),
                            code: error::errorList::error_s10.code,
                            message: error::errorList::error_s10.message.clone(),
                            builded_message: error::BuildedError::build_from_string(
                                error::errorList::error_s10.message.clone(),
                            ),
                            pos: functiondata.data.parameters[last_entry - 1].data.name_pos,
                        });
                    }
                    if let definers::DefinerCollecting::Generic(name) =
                        &functiondata.data.parameters[last_entry - 1].data.rtype
                    {
                        if !parser.type_exists(name.rtype.clone()) {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "replace".to_string(),
                                title: error::errorList::error_s6.title.clone(),
                                code: error::errorList::error_s6.code,
                                message: error::errorList::error_s6.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s6.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: name.rtype.clone(),
                                    }],
                                ),
                                pos: functiondata.data.parameters[last_entry - 1].data.type_pos,
                            });
                        }
                    }
                    if let definers::DefinerCollecting::Function(function) =
                        itered_data.data.rtype.clone()
                    {
                        if !functiondata.data.parameters[last_entry - 1]
                            .data
                            .rtype
                            .clone()
                            .same_as(function.params[last_entry - 1].clone())
                        {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "qq".to_string(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: function.params[last_entry - 1].raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: functiondata.data.parameters[last_entry - 1]
                                                .data
                                                .rtype
                                                .raw_name(),
                                        },
                                    ],
                                ),
                                pos: functiondata.data.parameters[last_entry - 1].data.type_pos,
                            });
                        }
                    }
                    //If its type's comma dont stop collecting it
                    functiondata
                        .data
                        .parameters
                        .push(function::FunctionParameterCollector::default());
                } else {
                    if letter_char == ")" {
                        functiondata.data.parameters[last_entry - 1].child_brace -= 1;
                    } else if letter_char == "(" {
                        functiondata.data.parameters[last_entry - 1].child_brace += 1;
                    }
                    functiondata.data.parameters[last_entry - 1]
                        .data
                        .pos
                        .range_end = parser.pos.clone().skipChar(1);
                    if functiondata.data.parameters[last_entry - 1]
                        .data
                        .type_pos
                        .range_start
                        .is_zero()
                        && letter_char != " "
                    {
                        functiondata.data.parameters[last_entry - 1]
                            .data
                            .type_pos
                            .range_start = parser.pos;
                    }
                    functiondata.data.parameters[last_entry - 1]
                        .data
                        .type_pos
                        .range_end = parser.pos.clone().skipChar(1);
                    processors::definer_processor::collect_definer(
                        parser.clone(),
                        &mut functiondata.data.parameters[last_entry - 1].data.rtype,
                        errors,
                        letter_char.to_string(),
                        next_char,
                        last_char,
                    );
                }
            }
        } else if !functiondata.pointer_typed {
            if letter_char == ">" {
                functiondata.pointer_typed = true;
            } else if letter_char == "{" {
                if itered_data.data.dynamic {}
                if let definers::DefinerCollecting::Function(function) =
                    itered_data.data.rtype.clone()
                {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "qq".to_string(),
                        title: error::errorList::error_s3.title.clone(),
                        code: error::errorList::error_s3.code,
                        message: error::errorList::error_s3.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s3.message.clone(),
                            vec![
                                error::ErrorBuildField {
                                    key: "token1".to_string(),
                                    value: function.returning.raw_name(),
                                },
                                error::ErrorBuildField {
                                    key: "token2".to_string(),
                                    value: "void".to_string(),
                                },
                            ],
                        ),
                        pos: itered_data.data.type_pos,
                    });
                }
                functiondata.return_typed = true;
                functiondata.pointer_typed = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: "arrow_function".to_string(),
                    debug_message: "2cd9a819587f1984d25bd2a4d452e341".to_string(),
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
                        range_end: parser.pos.clone().skipChar(1),
                    },
                });
            }
        } else if !functiondata.return_typed {
            if letter_char == "{" && functiondata.data.return_type.is_definer_complete() {
                if let definers::DefinerCollecting::Generic(name) = &functiondata.data.return_type {
                    if !parser.type_exists(name.rtype.clone()) {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "replace".to_string(),
                            title: error::errorList::error_s6.title.clone(),
                            code: error::errorList::error_s6.code,
                            message: error::errorList::error_s6.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s6.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: name.rtype.clone(),
                                }],
                            ),
                            pos: functiondata.data.return_pos,
                        });
                    }
                }
                if let definers::DefinerCollecting::Function(function) =
                    itered_data.data.rtype.clone()
                {
                    if !functiondata
                        .data
                        .return_type
                        .clone()
                        .same_as(*function.returning.clone())
                    {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "qq".to_string(),
                            title: error::errorList::error_s3.title.clone(),
                            code: error::errorList::error_s3.code,
                            message: error::errorList::error_s3.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s3.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "token1".to_string(),
                                        value: function.returning.raw_name(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_string(),
                                        value: functiondata.data.return_type.raw_name(),
                                    },
                                ],
                            ),
                            pos: functiondata.data.return_pos,
                        });
                    }
                }
                functiondata.return_typed = true;
            } else {
                if functiondata.data.return_pos.range_start.is_zero() && letter_char != " " {
                    functiondata.data.return_pos.range_start = parser.pos;
                }
                functiondata.data.return_pos.range_end = parser.pos;
                processors::definer_processor::collect_definer(
                    parser.clone(),
                    &mut functiondata.data.return_type,
                    errors,
                    letter_char.to_string(),
                    next_char,
                    last_char,
                );
            }
        } else if letter_char == "}" && functiondata.brace_count == 0 {
            functiondata.complete = true;
        } else {
            if letter_char == "{" {
                functiondata.brace_count += 1;
            } else if letter_char == "}" && functiondata.brace_count != 0 {
                functiondata.brace_count -= 1;
            }

            let code_letter = if last_char.clone() == "\n" || last_char.clone() == "\r" {
                last_char + letter_char //Make sure we get the lines correctly
            } else {
                letter_char.to_string()
            };
            functiondata.code += &code_letter;
        }
    }
}
