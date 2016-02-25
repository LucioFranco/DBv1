use super::types::Types;

pub struct Column {
    col_type: Types
}

impl Column {
    pub fn new(c_type: Types) -> Self {
        Column { col_type: c_type }
    }

    pub fn size(&self) -> u32 {
        self.col_type.size()
    }
}
