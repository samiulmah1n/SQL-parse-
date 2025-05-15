use crate::tokenizer::Tokenizer;
use crate::statement::{Statement, TableColumn, DBType};

pub fn build_statement(input: &str) -> Result<Statement, String> {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize()?;
    if input.trim_start().to_uppercase().starts_with("SELECT") {
        Ok(Statement::Select {
            columns: vec!["*".to_string()],
            table: "table".to_string(),
            where_clause: None,
            order_by: None,
        })
    } else if input.trim_start().to_uppercase().starts_with("CREATE TABLE") {
        Ok(Statement::CreateTable {
            name: "table".to_string(),
            columns: vec![TableColumn {
                name: "id".to_string(),
                col_type: DBType::Int,
            }],
        })
    } else {
        Err("Unsupported statement".to_string())
    }
}