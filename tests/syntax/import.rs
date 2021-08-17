#[cfg(test)]
mod variable_tests {

    #[test]
    fn import_collected_with_no_error() {
        let code = "
            import string;
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
            |_, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[0].clone(), ellie_parser::parser::Collecting::Import(x) if x.path == "string")
                && parsed.syntax_errors.len() == 0
        );
    }

    #[test]
    fn import_cannot_resolve_if_module_not_supplied() {
        let code = "
            import string;
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
            |_, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let parsed = emulated_parser.map();
        assert!(
            parsed.syntax_errors.len() == 1
                && matches!(parsed.syntax_errors[0].clone(),x if x.code == 39 && x.builded_message.fields[0].value == "string".to_string())
        );
    }
}
