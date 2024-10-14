use crate::db::data_types::{DataType, Keyword};

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    QuotedIdentifier(String),
    Number(i64),
    String(String),
    Symbol(char),
    DataType(DataType),
    Semicolon,
}

// Tokenizer (lexer) breaks down the raw string. For example: "SELECT name FROM users WHERE age >
// 18" can be broken down into a list of tokens ([Keyword("SELECT"), Identifier("name"),
// Keyword("FROM"), Identifier("users"), Keyword("WHERE", Identifier("age"), Symbol('>'), Number(18))]),
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            'A'..='Z' | 'a'..='z' | '_' => {
                let word = consume_while(&mut chars, |c| c.is_alphanumeric() || c == '_');
                if let Some(keyword) = str_to_keyword(word.as_str()) {
                    tokens.push(Token::Keyword(keyword));
                } else if let Some(data_type) = str_to_data_type(word.as_str()) {
                    tokens.push(Token::DataType(data_type));
                } else {
                    tokens.push(Token::Identifier(word));
                }
            }
            '0'..='9' => {
                let number = consume_while(&mut chars, |c| c.is_numeric());
                tokens.push(Token::Number(number.parse().unwrap()));
            }
            '\'' => {
                chars.next(); // consume opening quote
                let string = consume_while(&mut chars, |c| c != '\'');
                if chars.next() != Some('\'') {
                    return Err("Unterminated string literal".to_string());
                }
                tokens.push(Token::String(string));
            }
            '"' => {
                chars.next(); // consume opening quote
                let identifier = consume_while(&mut chars, |c| c != '"');
                if chars.next() != Some('"') {
                    return Err("Unterminated quoted identifier".to_string());
                }
                tokens.push(Token::QuotedIdentifier(identifier));
            }
            ',' | '(' | ')' | '>' | '<' | '=' | '*' => {
                tokens.push(Token::Symbol(c));
                chars.next();
            }
            _ => return Err(format!("Unexpected character: {}", c)),
        }
    }

    Ok(tokens)
}

fn consume_while<F>(chars: &mut std::iter::Peekable<std::str::Chars>, predicate: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut result = String::new();
    while let Some(&c) = chars.peek() {
        if predicate(c) {
            result.push(c);
            chars.next();
        } else {
            break;
        }
    }
    result
}

fn str_to_data_type(s: &str) -> Option<DataType> {
    match s.to_uppercase().as_str() {
        "INTEGER" => Some(DataType::Integer),
        "TEXT" => Some(DataType::Text),
        "REAL" => Some(DataType::Real),
        "BLOB" => Some(DataType::Blob),
        "NULL" => Some(DataType::Null),
        "BOOLEAN" => Some(DataType::Boolean),
        "DATE" => Some(DataType::Date),
        "TIMESTAMP" => Some(DataType::Timestamp),
        "VARCHAR" => Some(DataType::Varchar),
        "CHAR" => Some(DataType::Char),
        "FLOAT" => Some(DataType::Float),
        "DOUBLE" => Some(DataType::Double),
        "DECIMAL" => Some(DataType::Decimal),
        _ => None,
    }
}

fn str_to_keyword(s: &str) -> Option<Keyword> {
    match s.to_uppercase().as_str() {
        "SELECT" => Some(Keyword::Select),
        "FROM" => Some(Keyword::From),
        "WHERE" => Some(Keyword::Where),
        "INSERT" => Some(Keyword::Insert),
        "INTO" => Some(Keyword::Into),
        "VALUES" => Some(Keyword::Values),
        "CREATE" => Some(Keyword::Create),
        "TABLE" => Some(Keyword::Table),
        "OR" => Some(Keyword::Or),
        "AND" => Some(Keyword::And),
        "JOIN" => Some(Keyword::Join),
        "ON" => Some(Keyword::On),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_statement() {
        let input: &str = "SELECT * FROM users;";

        let result: Result<Vec<Token>, String> = tokenize(input);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                Token::Keyword(Keyword::Select),
                Token::Symbol('*'),
                Token::Keyword(Keyword::From),
                Token::Identifier(String::from("users")),
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn test_insert_statement() {
        let input: &str = "INSERT INTO users (id, name) VALUES (1, 'Tom');";

        let result: Result<Vec<Token>, String> = tokenize(input);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                Token::Keyword(Keyword::Insert),
                Token::Keyword(Keyword::Into),
                Token::Identifier(String::from("users")),
                Token::Symbol('('),
                Token::Identifier(String::from("id")),
                Token::Symbol(','),
                Token::Identifier(String::from("name")),
                Token::Symbol(')'),
                Token::Keyword(Keyword::Values),
                Token::Symbol('('),
                Token::Number(1),
                Token::Symbol(','),
                Token::String(String::from("Tom")),
                Token::Symbol(')'),
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn test_create_statement() {
        let input: &str = "CREATE TABLE my_table (id INTEGER, name TEXT);";

        let result: Result<Vec<Token>, String> = tokenize(input);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                Token::Keyword(Keyword::Create),
                Token::Keyword(Keyword::Table),
                Token::Identifier("my_table".to_string()),
                Token::Symbol('('),
                Token::Identifier("id".to_string()),
                Token::DataType(DataType::Integer),
                Token::Symbol(','),
                Token::Identifier("name".to_string()),
                Token::DataType(DataType::Text),
                Token::Symbol(')'),
                Token::Semicolon,
            ]
        )
    }
}
