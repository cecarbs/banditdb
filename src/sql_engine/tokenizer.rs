#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    QuotedIdentifier(String),
    Number(i64),
    String(String),
    Symbol(char),
    DataType(String),
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
                if [
                    "SELECT", "FROM", "WHERE", "INSERT", "INTO", "VALUES", "CREATE", "TABLE",
                ].contains(&word.to_uppercase().as_str()) {
                    tokens.push(Token::Keyword(word.to_uppercase()));
                } else if [
                    "INTEGER", "TEXT", "REAL", "BLOB", "NULL", "BOOLEAN", "DATE", "TIMESTAMP",
                    "VARCHAR", "CHAR", "FLOAT", "DOUBLE", "DECIMAL"
                ].contains(&word.to_uppercase().as_str()) {
                    tokens.push(Token::DataType(word.to_uppercase()));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_statement() {
        let input: &str = "SELECT * FROM users;";

        let result: Result<Vec<Token>, String> = tokenize(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![
            Token::Keyword(String::from("SELECT")),
            Token::Symbol('*'),
            Token::Keyword(String::from("FROM")),
            Token::Identifier(String::from("users")),
        ]);
    }

    #[test]
    fn test_insert_statement() {
        let input: &str = "INSERT INTO users (id, name) VALUES (1, 'Tom');";

        let result: Result<Vec<Token>, String> = tokenize(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![
            Token::Keyword(String::from("INSERT")),
            Token::Keyword(String::from("INTO")),
            Token::Identifier(String::from("users")),
            Token::Symbol('('),
            Token::Identifier(String::from("id")),
            Token::Symbol(','),
            Token::Identifier(String::from("name")),
            Token::Symbol(')'),
            Token::Keyword(String::from("VALUES")),
            Token::Symbol('('),
            Token::Number(1),
            Token::Symbol(','),
            Token::String(String::from("Tom")),
            Token::Symbol(')'),
        ])
    }

    #[test]
    fn test_create_statement() {
        let input: &str = "CREATE TABLE my_table (id INTEGER, name TEXT);";

        let result: Result<Vec<Token>, String> = tokenize(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![
            Token::Keyword("CREATE".to_string()),
            Token::Keyword("TABLE".to_string()),
            Token::Identifier("my_table".to_string()),
            Token::Symbol('('),
            Token::Identifier("id".to_string()),
            Token::DataType("INTEGER".to_string()),
            Token::Symbol(','),
            Token::Identifier("name".to_string()),
            Token::DataType("TEXT".to_string()),
            Token::Symbol(')'),
            Token::Semicolon,
        ])
    }
}
