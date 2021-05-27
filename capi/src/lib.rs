extern crate ellie_core;
extern crate ellie_parser;

pub mod parser;
pub mod syntax;
pub mod error;
pub mod defs;

use std::str;
use std::slice;
use libc::{c_char, strlen};

#[no_mangle]
pub unsafe extern "C" fn parser_new(test: *const c_char, options: ellie_core::defs::ParserOptions) -> parser::Parsed {
    let parser = ellie_parser::parser::Parser::new(
        str::from_utf8_unchecked(slice::from_raw_parts(test as *const u8, (strlen(test)+1) as usize)).to_string(),
        ellie_core::defs::ParserOptions {
            functions: true,
            break_on_error: false,
            loops: true,
            dynamics: true,
            global_variables: true,
            collectives: true,
            variables: true,
        },
    );

    parser::Parsed::default()
}