use alloc::{borrow::ToOwned, vec};
use ellie_core::{definite::Converter, error};
use ellie_tokenizer::syntax::items::file_key::FileKey;

impl super::Processor for FileKey {
    fn process(self, parser: &mut super::Parser, page_id: u64) {
        if self.value.is_static() {
            parser.find_processed_page(page_id).unwrap().items.push(
                ellie_core::definite::items::Collecting::FileKey(self.to_definite()),
            );
        } else {
            parser
                .informations
                .push(&error::error_list::ERROR_S9.clone().build_with_path(
                    vec![],
                    "pcls_0x14".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    self.value_location,
                ));
        }
    }
}