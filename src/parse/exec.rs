use std::collections::HashMap;
use super::token;

#[derive(Debug, Clone)]
pub enum Query {
    Database(DbStmt),
    Table(TableStmt),
}

#[derive(Debug, Clone)]
pub enum TableStmt {
    Select(SelectStmt),
    Insert(InsertStmt),
}

#[derive(Debug, Clone)]
pub enum DbStmt {
    CreateTable,
}

#[derive(Debug, Clone)]
pub struct InsertStmt {
    pub table: Table,
    pub cols: HashMap<Col, token::Lit>,
}

#[derive(Debug, Clone)]
pub struct SelectStmt {
    pub cols: Vec<Col>,
    pub table: Table, // TODO: add support for multipule tables
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Col {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub alias: Option<String>,
}
