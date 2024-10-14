use std::{iter::Peekable, slice::Iter};

use super::{tokenizer::Token, Join};
use crate::db::data_types::{Column, DataType, Keyword, Value};
// use crate::sql_engine::tokenizer::Token::Keyword;
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
use super::{Condition, SqlCommand};

type ParseResult<T> = Result<T, String>;

pub fn parse(tokens: &[Token]) -> Result<SqlCommand, String> {
    match tokens.get(0) {
        // TODO: redo this
        Some(Token::Keyword(keyword)) => match keyword {
            Keyword::Create => parse_create_table(tokens),
            Keyword::Insert => parse_insert(tokens),
            Keyword::Select => parse_select(tokens),
            _ => Err("Unable to parse token: unsupported command.".to_string()),
        },
        _ => Err("Invalid SQL command".to_string()),
    }
}

fn parse_create_table(tokens: &[Token]) -> Result<SqlCommand, String> {
    let mut iter = tokens.iter().peekable();

    loop {
        if iter.peek().is_none() {
            return Err(String::from(
                "Parser: expected CREATE TABLE keywords, but found none.",
            ));
        }
        match iter.next() {
            Some(Token::Keyword(Keyword::Create)) => continue,
            Some(Token::Keyword(Keyword::Table)) => break,
            _ => return Err("Parser: unexpected token. Expected CREATE TABLE keywords".to_string()),
        }
    }

    if iter.peek().is_none() {
        return Err(String::from("Parser: expected table name, but found none."));
    }
    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("Expected table name".to_string()),
    };

    if iter.peek().is_none() {
        return Err(String::from("Parser: column name, but found none."));
    }
    let mut columns: Vec<String> = Vec::new();
    let mut data_types: Vec<DataType> = Vec::new();
    loop {
        if iter.peek().is_none() {
            return Err(String::from(
                "Parser: expected column name or data type but found none.",
            ));
        }
        match iter.next() {
            Some(Token::Symbol('(')) => continue,
            Some(Token::Identifier(name)) => {
                columns.push(name.clone());

                if let Some(Token::DataType(type_name)) = iter.next() {
                    data_types.push(type_name.to_owned());
                } else {
                    return Err("Expected a data type after column name".to_string());
                }
            }
            Some(Token::Symbol(',')) => continue,
            Some(Token::Symbol(')')) => break,
            _ => return Err("Expected a column name or data type.".to_string()),
        }
    }

    if iter.peek().is_some() {
        match iter.next() {
            Some(Token::Semicolon) => {}
            Some(_) => return Err("Unexpected tokens after SEMICOLON".to_string()),
            None => {}
        }
    }

    let column_vec: Vec<Column> = columns
        .into_iter()
        .zip(data_types)
        .map(|(column_name, data_type)| Column {
            name: column_name,
            data_type,
        })
        .collect();
    Ok(SqlCommand::CreateTable {
        name: table_name,
        columns: column_vec,
    })
}

#[allow(unused_variables)]
fn parse_insert(tokens: &[Token]) -> Result<SqlCommand, String> {
    let mut iter = tokens.iter().peekable();

    // Expect INSERT keyword
    match iter.next() {
        Some(Token::Keyword(Keyword::Insert)) => {}
        _ => return Err("Expected INSERT keyword".to_string()),
    }

    // Expect INTO keyword
    match iter.next() {
        Some(Token::Keyword(Keyword::Into)) => {}
        _ => return Err("Expected INTO keyword".to_string()),
    }

    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("Expected table name".to_string()),
    };

    // Parse column values
    let mut columns = Vec::new();
    loop {
        match iter.next() {
            Some(Token::Symbol('(')) => continue,
            Some(Token::Identifier(name)) => columns.push(name.clone()),
            Some(Token::Symbol(',')) => continue,
            Some(Token::Symbol(')')) => break,
            _ => return Err("Expected column name or FROM keyword".to_string()),
        }
    }

    // Expect VALUES keyword
    match iter.next() {
        Some(Token::Keyword(keyword)) => {}
        _ => return Err("Expected VALUES keyword".to_string()),
    }

    // Parse values for each column
    let mut values = Vec::new();
    loop {
        match iter.next() {
            Some(Token::Symbol('(')) => continue,
            Some(Token::Symbol('\'')) => continue,
            Some(Token::Identifier(name)) => values.push(name.clone()),
            Some(Token::Symbol(',')) => continue,
            Some(Token::Symbol(')')) => break,
            _ => return Err("Expected column name or FROM keyword".to_string()),
        }
    }

    // Ensure we've consumed all tokens except for a possible semicolon
    match iter.next() {
        Some(Token::Semicolon) => {}
        Some(_) => return Err("Unexpected tokens after WHERE clause".to_string()),
        None => {}
    }

    Ok(SqlCommand::Insert {
        table: table_name,
        columns,
        values,
    })
}

fn parse_select(tokens: &[Token]) -> Result<SqlCommand, String> {
    let mut iter = tokens.iter().peekable();

    if iter.peek().is_none() {
        return Err("Parser: expected SELECT keyword, but found none.".to_string());
    }
    // Expect SELECT keyword
    match iter.next() {
        Some(Token::Keyword(Keyword::Select)) => {}
        _ => return Err("Expected SELECT keyword".to_string()),
    }

    if iter.peek().is_none() {
        return Err("Parser: expected column names, but found none.".to_string());
    }
    // Parse column names
    let mut columns = Vec::new();
    loop {
        if iter.peek().is_none() {
            return Err("Parser: error selecting column names. Expected '*' or column names, but found none.".to_string());
        }
        match iter.next() {
            Some(Token::Identifier(name)) => columns.push(name.clone()),
            Some(Token::QuotedIdentifier(name)) => columns.push(name.clone()),
            Some(Token::Symbol('*')) => columns.push("*".to_string()),
            Some(Token::Keyword(Keyword::From)) => break,
            Some(Token::Symbol(',')) => continue,
            _ => return Err("Expected column name or FROM keyword".to_string()),
        }
    }

    if iter.peek().is_none() {
        return Err("Parser: expected table name, but found none.".to_string());
    }
    // Parse table name
    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        Some(Token::QuotedIdentifier(name)) => name.clone(),
        _ => return Err("Expected table name".to_string()),
    };

    let mut where_clause = None;
    let join_clause = None;

    match iter.peek() {
        Some(Token::Keyword(Keyword::Where)) => {
            iter.next(); // consume WHERE keyword
            where_clause = Some(parse_where_clause(&mut iter)?)
        }
        Some(Token::Keyword(Keyword::Join)) => {
            iter.next();
            //TODO: implement inner join
            Some(parse_inner_join_clause(&mut iter)?);
        }
        Some(Token::Semicolon) => println!("Parser: end of SELECT statement."),
        Some(_) => {
            return Err(
                "Parser: unexpected token after table name. Expected WHERE, JOIN, or SEMICOLON."
                    .to_string(),
            )
        }
        None => {
            println!("Parser: end of SELECT statement.");
        }
    }

    Ok(SqlCommand::Select {
        table: table_name,
        columns,
        where_clause,
        join_clause,
    })
}

fn parse_inner_join_clause(iter: &mut Peekable<Iter<'_, Token>>) -> Result<Join, String> {
    todo!()
}

fn parse_where_clause(iter: &mut Peekable<Iter<'_, Token>>) -> Result<Vec<Condition>, String> {
    let mut conditions: Vec<Condition> = Vec::new();

    loop {
        let left = match iter.next() {
            Some(Token::Identifier(name)) | Some(Token::QuotedIdentifier(name)) => name.clone(),
            Some(Token::Number(n)) => n.to_string(),
            Some(Token::String(s)) => s.clone(),
            Some(unexpected) => return Err(format!("Parser: Unexpected token in WHERE clause: expected identifier or value, but got {:?}", unexpected)),
            None => return Err("Parser: Unexpected end of input in WHERE clause: expected identifier or value".to_string()),
        };

        let operator = match iter.next() {
            Some(Token::Symbol('=')) => "=".to_string(),
            Some(Token::Symbol('>')) => ">".to_string(),
            Some(Token::Symbol('<')) => "<".to_string(),
            Some(Token::Symbol('!')) => {
                if let Some(Token::Symbol('=')) = iter.next() {
                    "!=".to_string()
                } else {
                    return Err("Parser: expected '=' after '!' in WHERE clause".to_string());
                }
            }
            _ => return Err("Parser: expected comparison operator in WHERE clause".to_string()),
        };

        let right = match iter.next() {
            Some(Token::Identifier(name)) | Some(Token::QuotedIdentifier(name)) => name.clone(),
            Some(Token::Number(n)) => n.to_string(),
            Some(Token::String(s)) => s.clone(),
            Some(unexpected) => return Err(format!("Parser: unexpected token in WHERE clause: expected identifier or value, but got {:?}", unexpected)),
            None => return Err("Parser: unexpected end of input in WHERE clause: expected identifier or value".to_string()),
        };

        conditions.push(Condition::Comparison {
            left,
            operator,
            right,
        });

        match iter.peek() {
            Some(Token::Keyword(Keyword::And)) | Some(Token::Keyword(Keyword::Or)) => {
                // TODO: handle AND / OR cases
                iter.next(); // consume the AND/OR keyword
            }
            // TODO: create a function that handles the left and right comparisons
            Some(unexpected) => {
                println!(
                    "Parser Debug: Unexpected token after condition: {:?}",
                    unexpected
                );
                break; // If no AND/OR, we're done with the WHERE clause
            }
            None => break, // End of input, we're done with the WHERE clause
        }
    }

    Ok(conditions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_select_basic() {
        let tokens = vec![
            Token::Keyword(Keyword::Select),
            Token::Identifier("name".to_string()),
            Token::Symbol(','),
            Token::Identifier("age".to_string()),
            Token::Keyword(Keyword::From),
            Token::Identifier("users".to_string()),
        ];

        let result = parse_select(&tokens);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            SqlCommand::Select {
                table: "users".to_string(),
                columns: vec!["name".to_string(), "age".to_string()],
                where_clause: None,
                join_clause: None,
            }
        );
    }

    #[test]
    fn test_parse_select_with_where() {
        let tokens = vec![
            Token::Keyword(Keyword::Select),
            Token::Identifier("name".to_string()),
            Token::Keyword(Keyword::From),
            Token::Identifier("users".to_string()),
            Token::Keyword(Keyword::Where),
            Token::Identifier("age".to_string()),
            Token::Symbol('>'),
            Token::Number(18),
            Token::Semicolon,
        ];

        let result = parse_select(&tokens);
        assert!(result.is_ok());

        match result.unwrap() {
            SqlCommand::Select {
                table,
                columns,
                where_clause,
                join_clause,
            } => {
                assert_eq!(table, "users".to_string());
                assert_eq!(columns, vec!["name".to_string()]);
                assert!(where_clause.is_some());

                let conditions = where_clause.unwrap();
                assert_eq!(conditions.len(), 1);

                match &conditions[0] {
                    Condition::Comparison {
                        left,
                        operator,
                        right,
                    } => {
                        assert_eq!(left, "age");
                        assert_eq!(operator, ">");
                        assert_eq!(right, "18");
                    }
                    _ => panic!("Parser: expected Comparison condition"),
                }
            }
            _ => panic!("Parser: expected Select command"),
        }
    }

    // #[test]
    // fn test_parse_select_with_complex_where() {
    //     let tokens = vec![
    //         Token::Keyword(Keyword::Select),
    //         Token::Identifier("name".to_string()),
    //         Token::Symbol(','),
    //         Token::Identifier("email".to_string()),
    //         Token::Keyword(Keyword::Select),
    //         Token::Identifier("users".to_string()),
    //         Token::Keyword(Keyword::Where),
    //         Token::Identifier("age".to_string()),
    //         Token::Symbol('>'),
    //         Token::Number(18),
    //         Token::Keyword(Keyword::And),
    //         Token::Identifier("status".to_string()),
    //         Token::Symbol('='),
    //         Token::String("active".to_string()),
    //         Token::Semicolon,
    //     ];
    //
    //     let result = parse_select(&tokens);
    //     assert!(result.is_ok());
    //
    //     match result.unwrap() {
    //         SqlCommand::Select {
    //             table,
    //             columns,
    //             where_clause,
    //         } => {
    //             assert_eq!(table, "users".to_string());
    //             assert_eq!(columns, vec!["name".to_string(), "email".to_string()]);
    //             assert!(where_clause.is_some());
    //
    //             let conditions = where_clause.unwrap();
    //             assert_eq!(conditions.len(), 2);
    //
    //             match &conditions[0] {
    //                 Condition::Comparison {
    //                     left,
    //                     operator,
    //                     right,
    //                 } => {
    //                     assert_eq!(left, "age");
    //                     assert_eq!(operator, ">");
    //                     assert_eq!(right, "18");
    //                 }
    //                 _ => panic!("Expected Comparison condition"),
    //             }
    //
    //             match &conditions[1] {
    //                 Condition::Comparison {
    //                     left,
    //                     operator,
    //                     right,
    //                 } => {
    //                     assert_eq!(left, "status");
    //                     assert_eq!(operator, "=");
    //                     assert_eq!(right, "active");
    //                 }
    //                 _ => panic!("Expected Comparison condition"),
    //             }
    //         }
    //         _ => panic!("Expected Select command"),
    //     }
    // }

    #[test]
    fn test_parse_select_error() {
        let tokens = vec![
            Token::Keyword(Keyword::Insert),
            Token::Identifier("name".to_string()),
            Token::Keyword(Keyword::From),
            Token::Identifier("users".to_string()),
            Token::Semicolon,
        ];

        let result = parse_select(&tokens);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expected SELECT keyword");
    }

    #[test]
    fn test_parse_insert_basic() {
        let tokens = vec![
            Token::Keyword(Keyword::Insert),
            Token::Keyword(Keyword::Into),
            Token::Identifier("my table".to_string()),
            Token::Symbol('('),
            Token::Identifier("users".to_string()),
            Token::Symbol(')'),
            Token::Keyword(Keyword::Values),
            Token::Symbol('('),
            Token::Symbol('\''),
            Token::Identifier("charles".to_string()),
            Token::Symbol('\''),
            Token::Symbol(')'),
            Token::Semicolon,
        ];

        let result = parse_insert(&tokens);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            SqlCommand::Insert {
                table: "my table".to_string(),
                columns: vec!["users".to_string()],
                values: vec!["charles".to_string()],
            }
        )
    }

    #[test]
    fn test_parse_insert_error() {
        let tokens = vec![
            Token::Keyword(Keyword::Select),
            Token::Identifier("name".to_string()),
            Token::Keyword(Keyword::From),
            Token::Identifier("users".to_string()),
            Token::Semicolon,
        ];

        let result = parse_insert(&tokens);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expected INSERT keyword");
    }

    #[test]
    fn test_parse_create_table() {
        let tokens = vec![
            Token::Keyword(Keyword::Create),
            Token::Keyword(Keyword::Table),
            Token::Identifier("my_table".to_string()),
            Token::Symbol('('),
            Token::Identifier("users".to_string()),
            Token::DataType(DataType::Varchar),
            Token::Symbol(','),
            Token::Identifier("id".to_string()),
            Token::DataType(DataType::Integer),
            Token::Symbol(')'),
            Token::Semicolon,
        ];

        let columns: Vec<Column> = vec![
            Column::new(String::from("users"), DataType::Varchar ),
            Column::new(String::from("id"), DataType::Integer),
        ];

        let result = parse_create_table(&tokens);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SqlCommand::CreateTable {
            name: String::from("my_table"),
            columns,
        });
    }

    #[test]
    fn test_parse_create_table_create_token_error() {
        let tokens = vec![
            Token::Keyword(Keyword::Select),
            Token::Keyword(Keyword::Table),
            Token::Identifier("my_table".to_string()),
            Token::Symbol('('),
            Token::Identifier("users".to_string()),
            Token::DataType(DataType::Varchar),
            Token::Symbol(','),
            Token::Identifier("id".to_string()),
            Token::DataType(DataType::Integer),
            Token::Symbol(')'),
            Token::Semicolon,
        ];
        let result = parse_create_table(&tokens);

        assert!(result.is_err());
    }
}
