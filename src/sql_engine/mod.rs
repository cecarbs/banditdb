pub mod parser;
pub mod tokenizer;

// pub use parser::{parse_create_table, parse_insert, parse_select};
// pub use tokenizer::Token;

use crate::db::data_types::{Column, Value};

pub fn process_sql(input: &str) -> Result<SqlCommand, String> {
    let tokens = tokenizer::tokenize(input);
    parser::parse(&tokens)
}

// Use struct like enum to hold the data and represent the different types of operations useful to
// the db
pub enum SqlCommand {
    CreateTable {
        name: String,
        columns: Vec<Column>,
    },
    Insert {
        table: String,
        values: Vec<Value>,
    },
    Select {
        table: String,
        columns: Vec<String>,
        where_clause: Option<String>,
    },
    // Add other command types as needed
}
