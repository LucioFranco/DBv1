use bincode::rustc_serialize::{encode_into, decode_from};

#[derive(Clone, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Types {
    Int,
    Float,
    Bool,
    Char(u16),
}

impl Types {
    pub fn size(&self) -> u32 {
        match self {
            &Types::Int => 4u32,
            &Types::Float => 4u32,
            &Types::Bool => 1u32,
            &Types::Char(len) => (len) as u32,
        }
    }
}
