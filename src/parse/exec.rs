#[derive(Debug, Clone)]
pub enum Query {
    DbStmt(DbStmt),
    TableStmt(TableStmt),
}

#[derive(Debug, Clone)]
pub enum TableStmt {
    SelectStmt,
}

#[derive(Debug, Clone)]
pub enum DbStmt {
    CreateTable,
}
