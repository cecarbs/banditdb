use crate::db::data_types::{Column, DataType, Value};

use super::tokenizer::Token;
// pub struct Repl {}
//
// impl Repl {
//     pub fn parse_create_table(command: &str) -> Option<(String, Vec<Column>)> {
//         let parts: Vec<&str> = command.split_whitespace().collect();
//         if parts.len() < 4
//             || parts[0].to_lowercase() != "create"
//             || parts[1].to_lowercase() != "table"
//         {
//             return None;
//         }
//
//         let table_name = parts[2].to_string();
//         let column_str = command.split('(').nth(1)?.trim_end_matches(')');
//         let column_defs: Vec<&str> = column_str.split(',').map(|s| s.trim()).collect();
//
//         let columns = column_defs
//             .into_iter()
//             .filter_map(|def| {
//                 let parts: Vec<&str> = def.split_whitespace().collect();
//                 if parts.len() != 2 {
//                     return None;
//                 }
//                 let name = parts[0].to_string();
//                 let data_type = match parts[1].to_lowercase().as_str() {
//                     "integer" => DataType::Integer,
//                     "text" => DataType::Text,
//                     // Add more data type parsing as needed
//                     _ => return None,
//                 };
//                 Some(Column { name, data_type })
//             })
//             .collect();
//
//         Some((table_name, columns))
//     }
//
//     pub fn parse_insert(command: &str) -> Option<(String, Vec<Value>)> {
//         let parts: Vec<&str> = command.split_whitespace().collect();
//         if parts.len() < 4
//             || parts[0].to_lowercase() != "insert"
//             || parts[1].to_lowercase() != "into"
//         {
//             return None;
//         }
//
//         let table_name = parts[2].to_string();
//         let values_str = command.split('(').nth(1)?.trim_end_matches(')');
//         let value_strs: Vec<&str> = values_str.split(',').map(|s| s.trim()).collect();
//
//         let values = value_strs
//             .into_iter()
//             .map(|v| {
//                 if let Ok(i) = v.parse::<i64>() {
//                     Value::Integer(i)
//                 } else {
//                     Value::Text(v.trim_matches('"').to_string())
//                 }
//             })
//             .collect();
//
//         Some((table_name, values))
//     }
// }
use super::SqlCommand;

pub fn parse(tokens: &[Token]) -> Result<SqlCommand, String> {
    match tokens.get(0) {
        Some(Token::Keyword(keyword)) => match keyword.as_str() {
            "CREATE" => parse_create_table(tokens),
            "INSERT" => parse_insert(tokens),
            "SELECT" => parse_select(tokens),
            _ => Err(format!("Unsupported command: {}", keyword)),
        },
        _ => Err("Invalid SQL command".to_string()),
    }
}

fn parse_create_table(tokens: &[Token]) -> Result<SqlCommand, String> {
    // Implementation for CREATE TABLE parsing
    // This will be similar to our previous parse_create_table function,
    // but operating on tokens instead of a string
    // ...
    todo!()
}

fn parse_insert(tokens: &[Token]) -> Result<SqlCommand, String> {
    // Implementation for INSERT parsing
    // ...
    todo!()
}

fn parse_select(tokens: &[Token]) -> Result<SqlCommand, String> {
    let mut iter = tokens.iter().peekable();

    // Expect SELECT keyword
    match iter.next() {
        Some(Token::Keyword(k)) if k == "SELECT" => {}
        _ => return Err("Expected SELECT keyword".to_string()),
    }

    // Parse column names
    let mut columns = Vec::new();
    loop {
        match iter.next() {
            Some(Token::Identifier(name)) => columns.push(name.clone()),
            Some(Token::Keyword(k)) if k == "FROM" => break,
            Some(Token::Symbol(',')) => continue,
            _ => return Err("Expected column name or FROM keyword".to_string()),
        }
    }

    // Parse table name
    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("Expected table name".to_string()),
    };

    // Parse WHERE clause (if present)
    let mut where_clause = None;
    if let Some(Token::Keyword(k)) = iter.peek() {
        if k == "WHERE" {
            iter.next(); // consume WHERE keyword
                         // Here you would parse the condition
                         // For simplicity, we'll just collect the rest as a string
            where_clause = Some(
                iter.map(|t| format!("{:?}", t))
                    .collect::<Vec<_>>()
                    .join(" "),
            );
        }
    }
    // The parser can now construct the SqlCommand variants based on the tokens it receives from
    // the tokenizera
    Ok(SqlCommand::Select {
        table: table_name,
        columns,
        where_clause: where_clause.map(|s| s.to_string()),
    })
    // Ok(SqlCommand::Select {
    //     table: table_name,
    //     columns,
    //     where_clause,
    // })
}
