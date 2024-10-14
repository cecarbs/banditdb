use sled::{Db, IVec};
use std::collections::HashMap;
use std::str;
use serde::{Serialize, Deserialize};
use bincode;
use crate::db::data_types::Column;

#[derive(Serialize, Deserialize, Debug)]
struct Table {
    name: String,
    columns: Vec<Column>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    values: HashMap<String, String>,
}

struct SimpleDatabase {
    db: Db,
}

impl SimpleDatabase {
    fn new(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(SimpleDatabase { db })
    }

    fn create_table(&self, table_name: &str, columns: Vec<Column>) -> Result<(), Box<dyn std::error::Error>> {
        let table = Table {
            name: table_name.to_string(),
            columns,
        };
        let table_data = bincode::serialize(&table)?;
        self.db.insert(format!("table:{}", table_name), table_data)?;
        self.db.flush()?;
        Ok(())
    }

    fn insert_row(&self, table_name: &str, row_data: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let table: Table = self.get_table(table_name)?;

        // Validate that all required columns are present
        for col in &table.columns {
            if !row_data.contains_key(col) {
                return Err(format!("Missing column: {}", col).into());
            }
        }

        let row = Row { values: row_data };
        let row_id = self.db.generate_id()?;
        let row_key = format!("{}:{}", table_name, row_id);
        let row_data = bincode::serialize(&row)?;

        self.db.insert(row_key, row_data)?;
        self.db.flush()?;
        Ok(())
    }

    fn get_table(&self, table_name: &str) -> Result<Table, Box<dyn std::error::Error>> {
        let table_data = self.db.get(format!("table:{}", table_name))?
            .ok_or("Table not found")?;
        let table: Table = bincode::deserialize(&table_data)?;
        Ok(table)
    }

    fn get_row(&self, table_name: &str, row_id: u64) -> Result<Row, Box<dyn std::error::Error>> {
        let row_key = format!("{}:{}", table_name, row_id);
        let row_data = self.db.get(row_key)?.ok_or("Row not found")?;
        let row: Row = bincode::deserialize(&row_data)?;
        Ok(row)
    }

    fn scan_table(&self, table_name: &str) -> impl Iterator<Item = Result<Row, Box<dyn std::error::Error>>> + '_ {
        self.db
            .scan_prefix(table_name)
            .filter_map(Result::ok)
            .filter(|(key, _)| !key.starts_with(b"table:"))
            .map(move |(_, value)| {
                bincode::deserialize(&value)
                    .map_err(|e| e.into())
            })
    }
}