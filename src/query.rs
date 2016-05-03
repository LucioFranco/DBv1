use super::parse;
use parse::ast::*;

pub fn execute_from_ast(q: Query) {
    match q {
        Query::Table(stmt) => execute_table_stmt(stmt),
        Query::Database(stmt) => execute_database_stmt(stmt),
    };
}

fn execute_table_stmt(stmt: TableStmt) {
    unimplemented!();
}

fn execute_database_stmt(stmt: DatabaseStmt) {
    unimplemented!();
}