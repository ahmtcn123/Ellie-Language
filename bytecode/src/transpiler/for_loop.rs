use ellie_core::definite::items::for_loop;

impl super::Transpiler for for_loop::ForLoop {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        std::println!("[Assembler,Ignore,Element] ForLoop");
        true
    }
}