use super::column::Column;

pub struct Row {
    columns: Vec<Column>
}

impl Row {
    fn new() -> Self {
        Row { columns: Vec::new() }
    }
}
