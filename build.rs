use ellie_core;
use ellie_parser::parser;
use regex::Regex;
#[path = "src/terminal_colors.rs"]
mod terminal_colors;
use serde_json;
use std::{
    fs::{self, File},
    io::Read,
};

fn read_file(file_dir: &str) -> Result<String, String> {
    let file_read = File::open(file_dir);
    match file_read {
        Err(r) => Err(r.to_string()),
        Ok(mut file) => {
            let mut file_content = Vec::new();
            file.read_to_end(&mut file_content).expect("Unable to read");
            let code_string = String::from_utf8(file_content);
            Ok(code_string.unwrap())
        }
    }
}

fn resolve_import(lib_name: String) -> ellie_parser::parser::ResolvedImport {
    std::eprintln!(
        "{}[ReadingFile]{}: {}~./lib/{}.ei{}",
        terminal_colors::get_color(terminal_colors::Colors::Magenta),
        terminal_colors::get_color(terminal_colors::Colors::Reset),
        terminal_colors::get_color(terminal_colors::Colors::Yellow),
        lib_name,
        terminal_colors::get_color(terminal_colors::Colors::Reset),
    );
    match read_file(&("./lib/".to_string() + &lib_name + &".ei".to_string())) {
        Ok(e) => ellie_parser::parser::ResolvedImport {
            found: true,
            file_content: parse(e, lib_name).parsed,
        },
        Err(err) => {
            panic!(
                "{}[Fail]{}: Cannot read file {}~./lib/{}.ei{}\n{:#?}",
                terminal_colors::get_color(terminal_colors::Colors::Red),
                terminal_colors::get_color(terminal_colors::Colors::Reset),
                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                lib_name,
                terminal_colors::get_color(terminal_colors::Colors::Reset),
                err
            );
        }
    }
}

fn parse(contents: String, file_name: String) -> ellie_parser::parser::ParserResponse {
    std::eprintln!(
        "{}[ParsingFile]{}: {}~./lib/{}.ei{}",
        terminal_colors::get_color(terminal_colors::Colors::Cyan),
        terminal_colors::get_color(terminal_colors::Colors::Reset),
        terminal_colors::get_color(terminal_colors::Colors::Yellow),
        file_name,
        terminal_colors::get_color(terminal_colors::Colors::Reset),
    );
    let parser = parser::Parser::new(
        contents.clone(),
        resolve_import,
        ellie_core::defs::ParserOptions {
            functions: true,
            break_on_error: false,
            loops: true,
            conditions: true,
            classes: true,
            dynamics: true,
            global_variables: true,
            line_ending: "\\n".to_string(),
            collectives: true,
            variables: true,
            constants: true,
            parser_type: ellie_core::defs::ParserType::RawParser,
            allow_import: true,
        },
    );
    let parsed = parser.map();

    if parsed.syntax_errors.len() == 0 {
        std::eprintln!(
            "{}[ParsingSuccess]{}: {}~./lib/{}.ei{}",
            terminal_colors::get_color(terminal_colors::Colors::Green),
            terminal_colors::get_color(terminal_colors::Colors::Reset),
            terminal_colors::get_color(terminal_colors::Colors::Yellow),
            file_name,
            terminal_colors::get_color(terminal_colors::Colors::Reset),
        );
    } else {
        panic!(
            "{}[ParsingFailed]{}: {}~./lib/{}.ei{}",
            terminal_colors::get_color(terminal_colors::Colors::Red),
            terminal_colors::get_color(terminal_colors::Colors::Reset),
            terminal_colors::get_color(terminal_colors::Colors::Yellow),
            file_name,
            terminal_colors::get_color(terminal_colors::Colors::Reset),
        );
    }
    parsed
}

fn main() {
    match read_file(&("./lib/ellie.ei".to_string())) {
        Ok(ellie_lib) => {
            match read_file(&("./core/src/builded_libraries.rs".to_string())) {
                Ok(current_lib) => {
                    //@version *=\\s*\"\\^|\\~?(\\d|x|\\*)+\\.(\\d|x|\\*)+\\.(\\d|x|\\*)
                    let version_line_regex = Regex::new(
                        "(@(\\s)*version(\\s)*=)(\\s)*(\")*(?P<version>\"\\^|\\~?(\\d|x|\\*)+\\.(\\d|x|\\*)+\\.(\\d|x|\\*))*(\"|()*;)",
                    )
                    .unwrap();

                    if version_line_regex.is_match(&ellie_lib.clone())
                        && version_line_regex.is_match(&current_lib.clone())
                    {
                        let lib_version_number = &version_line_regex.captures(&ellie_lib).unwrap();
                        let current_version_number =
                            &version_line_regex.captures(&current_lib).unwrap();

                        if lib_version_number.name("version").is_some()
                            && current_version_number.name("version").is_some()
                        {
                            let lib_version = &lib_version_number["version"];
                            let current_version = &current_version_number["version"];
                            if lib_version == current_version {
                                eprintln!(
                                    "\nCompiling Ellie standard library {}v{}{} is not required",
                                    terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                    lib_version,
                                    terminal_colors::get_color(terminal_colors::Colors::Reset),
                                );
                            } else {
                                let ellie_lib =
                                    parse(ellie_lib.clone(), "ellie".to_string()).parsed;

                                eprintln!(
                                    "\nCompiling Ellie standard library {}v{}{} complete",
                                    terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                    lib_version,
                                    terminal_colors::get_color(terminal_colors::Colors::Reset),
                                );
                                let j = serde_json::to_string(&ellie_lib).unwrap();
                                fs::write(
                                    "./core/src/builded_libraries.rs",
                                    format!("//@version = \"{}\";\npub static ELLIE_STANDARD_LIBRARY : &str = {:#?};", lib_version, j),
                                )
                                .unwrap();

                                panic!("Compile {} {}", lib_version, current_version);
                            }
                        } else if lib_version_number.name("version").is_some() {
                            panic!(
                                "\nCompiling Ellie Failed, No version key found in {}~./core/src/builded_libraries.rs{}",
                                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                terminal_colors::get_color(terminal_colors::Colors::Reset),
                            );
                        } else {
                            panic!(
                                "\nCompiling Ellie Failed, No version key found in {}~./lib/ellie.ei{}",
                                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                terminal_colors::get_color(terminal_colors::Colors::Reset),
                            );
                        }
                    } else {
                        if version_line_regex.is_match(&ellie_lib.clone()) {
                            panic!(
                                "\nCompiling Ellie Failed, No version key found in {}~./core/src/builded_libraries.rs{}",
                                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                terminal_colors::get_color(terminal_colors::Colors::Reset),
                            );
                        } else {
                            panic!(
                                "\nCompiling Ellie Failed, No version key found in {}~./lib/ellie.ei{}",
                                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                terminal_colors::get_color(terminal_colors::Colors::Reset),
                            );
                        }
                    }
                }
                Err(err) => {
                    panic!(
                        "{}[Fail]{}: Cannot read file {}~./core/{}.rs{}\n{:#?}",
                        terminal_colors::get_color(terminal_colors::Colors::Red),
                        terminal_colors::get_color(terminal_colors::Colors::Reset),
                        terminal_colors::get_color(terminal_colors::Colors::Yellow),
                        "builded_libraries",
                        terminal_colors::get_color(terminal_colors::Colors::Reset),
                        err
                    );
                }
            }
        }
        Err(err) => {
            panic!(
                "{}[Fail]{}: Cannot read file {}~./lib/{}.ei{}\n{:#?}",
                terminal_colors::get_color(terminal_colors::Colors::Red),
                terminal_colors::get_color(terminal_colors::Colors::Reset),
                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                "ellie",
                terminal_colors::get_color(terminal_colors::Colors::Reset),
                err
            );
        }
    }
}