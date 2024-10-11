#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(i64),
    String(String),
    Symbol(char),
}

// Tokenizer (lexer) breaks down the raw string. For example: "SELECT name FROM users WHERE age >
// 18" can be broken down into a list of tokens ([Keyword("SELECT"), Identifier("name"),
// Keyword("FROM"), Identifier("users"), Keyword("WHERE", Identifier("age"), Symbol('>'), Number(18))]),
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            'A'..='Z' | 'a'..='z' => {
                let word = consume_while(&mut chars, |c| c.is_alphabetic());
                if [
                    "SELECT", "FROM", "WHERE", "INSERT", "INTO", "VALUES", "CREATE", "TABLE",
                ]
                .contains(&word.to_uppercase().as_str())
                {
                    tokens.push(Token::Keyword(word.to_uppercase()));
                } else {
                    tokens.push(Token::Identifier(word));
                }
            }
            '0'..='9' => {
                let number = consume_while(&mut chars, |c| c.is_numeric());
                tokens.push(Token::Number(number.parse().unwrap()));
            }
            '"' => {
                chars.next(); // consume opening quote
                let string = consume_while(&mut chars, |c| c != '"');
                chars.next(); // consume closing quote
                tokens.push(Token::String(string));
            }
            ',' | '(' | ')' | '>' | '<' | '=' => {
                tokens.push(Token::Symbol(c));
                chars.next();
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    tokens
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
