#[derive(Debug)]
pub enum Statement {
    Select {
        columns: Vec<String>,
        table: String,
        where_clause: Option<String>,
        order_by: Option<String>,
    },
    CreateTable {
        name: String,
        columns: Vec<TableColumn>,
    },
}

#[derive(Debug)]
pub struct TableColumn {
    pub name: String,
    pub col_type: DBType,
}

#[derive(Debug)]
pub enum DBType {
    Int,
    Varchar(usize),
}