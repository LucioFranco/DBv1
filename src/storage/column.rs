use super::types::Types;
use bincode::rustc_serialize::{encode_into, decode_from};

#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Column {
    name: String,
    col_type: Types,
}

impl Column {
    pub fn new(name: &str, c_type: Types) -> Self {
        Column {
            name: name.to_string(),
            col_type: c_type,
        }
    }

    pub fn size(&self) -> u32 {
        self.col_type.size()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
