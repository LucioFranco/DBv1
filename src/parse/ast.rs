use std::collections::HashMap;
use super::token;

#[derive(Debug, Clone, PartialEq)]
pub enum Query {
    Database(DatabaseStmt),
    Table(TableStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableStmt {
    Select(SelectStmt),
    Insert(InsertStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseStmt {
    CreateTable,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InsertStmt {
    pub table: Table,
    pub cols: HashMap<Col, token::Lit>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectStmt {
    pub cols: Vec<Col>,
    pub table: Table, // TODO: add support for multipule tables
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Col {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub name: String,
    pub alias: Option<String>,
}
