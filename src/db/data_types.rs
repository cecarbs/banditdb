use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    Integer,
    Text,
    // Add more data types as needed
}

// Used for defining the schema
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
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
        match (self, data_type) {
            (Value::Integer(_), DataType::Integer) => true,
            (Value::Text(_), DataType::Text) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CaseInsensitiveString(pub String);

impl From<String> for CaseInsensitiveString {
    fn from(s: String) -> Self {
        CaseInsensitiveString(s.to_lowercase())
    }
}
