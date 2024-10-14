use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use super::data_types::{CaseInsensitiveString, Column, Value};
use super::table::{Row, Table};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    tables: BTreeMap<CaseInsensitiveString, Table>,
    next_row_id: usize,
}

impl Database {
    // TODO: change tables to use sled B-Tree, for now just log and make sure the commands are correct
    pub fn new() -> Self {
        Database {
            tables: BTreeMap::new(),
            next_row_id: 0,
        }
    }

    pub fn create_table(&mut self, table_name: String, columns: Vec<Column>) {
        let table = Table::new(columns);
        self.tables.insert(table_name.into(), table);
    }

    pub fn insert_row(&mut self, table_name: &str, values: Vec<Value>) -> Result<(), String> {
        let table = self
            .tables
            .get_mut(&CaseInsensitiveString(table_name.to_string()))
            .ok_or_else(|| format!("Table '{}' not found", table_name))?;

        let row_id = self.next_row_id;
        self.next_row_id += 1;

        table.insert_row(row_id, values)
    }

    pub fn select(&self, table_name: &str, columns: &[String]) -> Result<Vec<Row>, String> {
        let table = self
            .tables
            .get(&CaseInsensitiveString(table_name.to_string()))
            .ok_or_else(|| format!("Table '{}' not found", table_name))?;

        table.select(columns)
    }

    pub fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
        let serialized = serde_json::to_string(self)?;
        fs::write(path, serialized)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> std::io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let database: Database = serde_json::from_str(&contents)?;
        Ok(database)
    }

    pub fn get_table(&self, table_name: &str) -> Result<&Table, String> {
        self.tables
            .get(&CaseInsensitiveString(table_name.to_string()))
            .ok_or_else(|| format!("Table '{}' not found", table_name))
    }
}
