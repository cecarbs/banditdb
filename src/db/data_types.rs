// TODO: might not need this anymore since we have tokenizer
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    Integer,
    Text,
    Real,
    Blob,
    Null,
    Boolean,
    Date,
    Timestamp,
    Varchar,
    Char,
    Float,
    Double,
    Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Keyword {
    Select,
    From,
    Where,
    Insert,
    Into,
    Values,
    Create,
    Table,
    And,
    Or,
    Join,
    On,
}

// Used for defining the schema
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

impl Column {
    pub fn new(name: String, data_type: DataType) -> Column {
        Self { name, data_type }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ForeignKey {
    column: String,
    references: String,
    referenced_column: String,
}

// Used for defining the schema
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Integer(i64),
    Text(String),
    // Add more value types as needed
}

impl Value {
    pub fn matches_type(&self, data_type: &DataType) -> bool {
        true
        // match (self, data_type) {
        //     (Value::Integer(_), DataType::INTEGER) => true,
        //     (Value::Text(_), DataType::TEXT) => true,
        //     _ => false,
        // }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CaseInsensitiveString(pub String);

impl From<String> for CaseInsensitiveString {
    fn from(s: String) -> Self {
        CaseInsensitiveString(s.to_lowercase())
    }
}
