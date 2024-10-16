use serde::{Deserialize, Serialize};
use sled::{Db, IVec};
use std::collections::HashMap;

// Represent a column
#[derive(Clone, Serialize, Deserialize)]
struct Column {
    name: String,
    data_type: DataType,
}

// Supported data types
#[derive(Clone, Serialize, Deserialize)]
enum DataType {
    Integer,
    Float,
    String,
    Boolean,
}

// Represent a table
struct Table {
    name: String,
    columns: Vec<Column>,
    primary_key: String,
}

// Represent a database
struct Database {
    db: Db,
    tables: HashMap<String, Table>,
}

impl Database {
    fn new(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(Self {
            db,
            tables: HashMap::new(),
        })
    }

    fn create_table(
        &mut self,
        name: &str,
        columns: Vec<Column>,
        primary_key: &str,
    ) -> Result<(), String> {
        if self.tables.contains_key(name) {
            return Err(format!("Table '{}' already exists", name));
        }

        let table = Table {
            name: name.to_string(),
            columns,
            primary_key: primary_key.to_string(),
        };

        self.tables.insert(name.to_string(), table);
        Ok(())
    }

    fn insert(
        &self,
        table_name: &str,
        data: HashMap<String, serde_json::Value>,
    ) -> Result<(), String> {
        let table = self
            .tables
            .get(table_name)
            .ok_or_else(|| format!("Table '{}' not found", table_name))?;

        let primary_key_value = data
            .get(&table.primary_key)
            .ok_or_else(|| format!("Primary key '{}' not provided", table.primary_key))?;

        let serialized_data =
            serde_json::to_vec(&data).map_err(|e| format!("Failed to serialize data: {}", e))?;

        let key = format!("{}:{}", table_name, primary_key_value);
        self.db
            .insert(key.as_bytes(), serialized_data)
            .map_err(|e| format!("Failed to insert data: {}", e))?;

        Ok(())
    }

    fn get(
        &self,
        table_name: &str,
        primary_key_value: &str,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, String> {
        let key = format!("{}:{}", table_name, primary_key_value);
        let result = self
            .db
            .get(key.as_bytes())
            .map_err(|e| format!("Failed to get data: {}", e))?;

        match result {
            Some(ivec) => {
                let data: HashMap<String, serde_json::Value> = serde_json::from_slice(&ivec)
                    .map_err(|e| format!("Failed to deserialize data: {}", e))?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    // Add methods for updating and deleting records, querying, etc.
}

// Example usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Database::new("my_database")?;

    // Create tables
    db.create_table(
        "users",
        vec![
            Column {
                name: "id".to_string(),
                data_type: DataType::Integer,
            },
            Column {
                name: "name".to_string(),
                data_type: DataType::String,
            },
            Column {
                name: "age".to_string(),
                data_type: DataType::Integer,
            },
        ],
        "id",
    )?;

    db.create_table(
        "posts",
        vec![
            Column {
                name: "id".to_string(),
                data_type: DataType::Integer,
            },
            Column {
                name: "user_id".to_string(),
                data_type: DataType::Integer,
            },
            Column {
                name: "title".to_string(),
                data_type: DataType::String,
            },
            Column {
                name: "content".to_string(),
                data_type: DataType::String,
            },
        ],
        "id",
    )?;

    // Insert data
    let mut user_data = HashMap::new();
    user_data.insert("id".to_string(), serde_json::json!(1));
    user_data.insert("name".to_string(), serde_json::json!("Alice"));
    user_data.insert("age".to_string(), serde_json::json!(30));
    db.insert("users", user_data)?;

    let mut post_data = HashMap::new();
    post_data.insert("id".to_string(), serde_json::json!(1));
    post_data.insert("user_id".to_string(), serde_json::json!(1));
    post_data.insert("title".to_string(), serde_json::json!("First Post"));
    post_data.insert("content".to_string(), serde_json::json!("Hello, World!"));
    db.insert("posts", post_data)?;

    // Retrieve data
    let user = db.get("users", "1")?;
    println!("User: {:?}", user);

    let post = db.get("posts", "1")?;
    println!("Post: {:?}", post);

    Ok(())
}
