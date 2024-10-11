pub mod parser;
pub mod tokenizer;

// pub use parser::{parse_create_table, parse_insert, parse_select};
// pub use tokenizer::Token;

use crate::db::data_types::{Column, Value};

pub fn process_sql(input: &str) -> Result<SqlCommand, String> {
    let tokens = tokenizer::tokenize(input);
    match tokens {
        Ok(tokens) => parser::parse(&tokens),
        Err(e) => Err(e),
    }

}

// Use struct like enum to hold the data and represent the different types of operations useful to
// the db

#[derive(Debug, PartialEq)]
pub enum SqlCommand {
    CreateTable {
        name: String,
        columns: Vec<Column>,
    },
    Insert {
        table: String,
        columns: Vec<String>,
        values: Vec<String>,
    },
    Select {
        table: String,
        columns: Vec<String>,
        where_clause: Option<Vec<Condition>>,
    },
    // Add other command types as needed
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Comparison {
        left: String,
        operator: String,
        right: String,
    },
    // Add more variants as needed, e.g., for AND, OR, etc.
}
