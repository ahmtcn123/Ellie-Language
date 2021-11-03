use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::processors;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error, utils};

pub fn collect_setter_value<F, E>(
    parser: &mut parser::Parser<F, E>,
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
    let parser_clone = parser.clone();
    if let parser::Collecting::Setter(ref mut setter_data) = parser.current {
        if !setter_data.name_wrote {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n") || setter_data.data.name.is_empty())
            {
                if setter_data.data.name.is_empty() {
                    setter_data.data.name_pos.range_start = parser.pos;
                }
                setter_data.data.name += letter_char;
                setter_data.data.name_pos.range_end = parser.pos.clone().skip_char(1);
            } else if letter_char == ":" && !setter_data.data.name.is_empty() {
                if utils::is_reserved(&setter_data.data.name) {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "290474bc378e5dbf7063790c6872a75f".to_owned(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: setter_data.data.name.clone(),
                            }],
                        ),
                        pos: setter_data.data.name_pos,
                    });
                }
                setter_data.name_wrote = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "b728199700d370e5a55a1e87c25cda6b".to_owned(),
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
        } else if !setter_data.type_wrote {
            if setter_data.data.rtype.is_definer_complete() && letter_char == "{" {
                setter_data.type_wrote = true;
                setter_data.data.bracket_start_pos.range_start = parser.pos;
                setter_data.data.bracket_start_pos.range_end = parser.pos.clone().skip_char(1);
            } else {
                if setter_data.data.rtype_pos.range_start.0 == 0
                    && setter_data.data.rtype_pos.range_start.1 == 0
                    && letter_char != " "
                {
                    setter_data.data.rtype_pos.range_start = parser.pos;
                }
                processors::definer_processor::collect_definer(
                    parser_clone,
                    &mut setter_data.data.rtype,
                    errors,
                    letter_char.to_string(),
                    next_char,
                    last_char,
                );
                setter_data.data.rtype_pos.range_end = parser.pos;
            }
        } else if !setter_data.param_bracket_opened
            && setter_data.brace_count == 0
            && letter_char == "}"
        {
            let mut filtered_items: Vec<parser::Collecting> = Vec::new();
            let mut ret_found = false;
            for item in setter_data.inside_code.collected.clone() {
                match item {
                    parser::Collecting::ImportItem(e) => {
                        if e.from_path != "<temporary>" {
                            filtered_items.push(parser::Collecting::ImportItem(e))
                        }
                    }
                    parser::Collecting::Ret(return_item) => {
                        if !ret_found {
                            ret_found = true;
                            if return_item.value.clone().to_definer() != setter_data.data.rtype {
                                errors.push(error::Error {
                                    path: parser.options.path.clone(),
                                    scope: parser.scope.scope_name.clone(),
                                    debug_message: "replace_setter_121".to_owned(),
                                    title: error::errorList::error_s3.title.clone(),
                                    code: error::errorList::error_s3.code,
                                    message: error::errorList::error_s3.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s3.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token2".to_owned(),
                                                value: return_item
                                                    .value
                                                    .clone()
                                                    .to_definer()
                                                    .raw_name_with_extensions(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token1".to_owned(),
                                                value: setter_data
                                                    .data
                                                    .rtype
                                                    .raw_name_with_extensions(),
                                            },
                                        ],
                                    ),
                                    pos: return_item.pos,
                                });
                            }
                        }
                        filtered_items.push(parser::Collecting::Ret(return_item))
                    }
                    e => filtered_items.push(e),
                }
            }

            if !ret_found && setter_data.data.rtype.raw_name() != "void" {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "replace_setter_159".to_owned(),
                    title: error::errorList::error_s3.title.clone(),
                    code: error::errorList::error_s3.code,
                    message: error::errorList::error_s3.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s3.message.clone(),
                        vec![
                            error::ErrorBuildField {
                                key: "token2".to_owned(),
                                value: "void".to_owned(),
                            },
                            error::ErrorBuildField {
                                key: "token1".to_owned(),
                                value: setter_data.data.rtype.raw_name_with_extensions(),
                            },
                        ],
                    ),
                    pos: setter_data.data.rtype_pos,
                });
            }

            setter_data.data.code = filtered_items;
            setter_data.inside_code = Box::new(parser::RawParser::default()); //Empty the cache
            setter_data.data.bracket_start_pos.range_start = parser.pos;
            setter_data.data.bracket_start_pos.range_end = parser.pos.clone().skip_char(1);
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else if !setter_data.param_bracket_opened {
            if letter_char == "{" {
                setter_data.brace_count += 1;
            } else if letter_char == "}" && setter_data.brace_count != 0 {
                setter_data.brace_count -= 1;
            }
            let mut child_parser = setter_data.inside_code.clone().to_no_resolver_parser();

            if setter_data.inside_code.pos.is_zero() {
                //Make sure upper scope imported once

                for item in parser.collected.clone() {
                    //Import variables as temporary for syntax support, we will remove them after collecting complete
                    child_parser.collected.push(parser::Collecting::ImportItem(
                        crate::syntax::import_item::ImportItem {
                            from_path: "<temporary>".to_owned(),
                            resolution_id: 0,
                            from_import: 0,
                            public: true,
                            item: Box::new(item),
                        },
                    ));
                }
            }

            child_parser.options = parser.options.clone();
            child_parser.options.parser_type = defs::ParserType::RawParser;
            child_parser.pos = parser.pos;
            child_parser.scope.scope_name = "core/function_processor".to_owned();
            child_parser.current = setter_data.inside_code.current.clone();
            child_parser.keyword_catch = setter_data.inside_code.keyword_catch.clone();
            child_parser.keyword_cache = setter_data.inside_code.keyword_cache.clone();

            let mut child_parser_errors: Vec<error::Error> = Vec::new();
            parser::iterator::iter(
                &mut child_parser,
                &mut child_parser_errors,
                letter_char,
                next_char,
                last_char,
            );
            for i in child_parser_errors {
                errors.push(i);
            }
            setter_data.inside_code = Box::new(child_parser.to_raw());
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
