#[cfg(test)]
mod ret_tests {

    #[test]
    fn ret_collected_with_no_error() {
        let code = "
            ret \"ok\";
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions {
                path: "<virtual>".to_string(),
                functions: true,
                break_on_error: true,
                loops: true,
                enums: true,
                classes: true,
                getters: true,
                setters: true,
                conditions: true,
                global_variables: true,
                line_ending: "\n\r".to_string(),
                dynamics: true,
                collectives: true,
                variables: true,
                import_std: false,
                constants: true,
                parser_type: ellie_core::defs::ParserType::RawParser,
                allow_import: true,
            }
        );
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[0].clone(), ellie_parser::parser::Collecting::Ret(x) if x.value.get_type() == "string")
                && parsed.syntax_errors.len() == 0
        );
    }
}
