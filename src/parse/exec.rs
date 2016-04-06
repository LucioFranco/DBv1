#[derive(Debug, Clone)]
pub enum Query {
    DbStmt(DbStmt),
    TableStmt(TableStmt),
}

#[derive(Debug, Clone)]
pub enum TableStmt {
    Select(SelectStmt),
}

#[derive(Debug, Clone)]
pub enum DbStmt {
    CreateTable,
}

#[derive(Debug, Clone)]
pub struct SelectStmt {
    pub cols: Vector<Col>,
    pub table: Table, // TODO: add support for multipule tables
}

#[derive(Debug, Clone)]
pub struct Col {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub alias: Option<String>,
}
