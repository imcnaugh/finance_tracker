use crate::NewLineItem;
use std::error::Error;

pub struct LineItemService {}

impl LineItemService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log_new_line_item(new_line_item: NewLineItem) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
