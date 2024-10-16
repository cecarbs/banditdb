use std::{collections::HashMap, error::Error};

use serde::Serialize;
use sled::Db;

use super::data_types::{Column, ForeignKey};

#[derive(Serialize)]
struct Table {
    name: String,
    columns: Vec<Column>,
    foreign_keys: Vec<ForeignKey>,
}

struct Database {
    db: Db,
    tables: HashMap<String, Table>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(Self {
            db,
            tables: HashMap::new(),
        })
    }

    pub fn create_table(&mut self, table: Table) -> Result<(), Box<dyn Error>> {
        let table_name = table.name.clone();
        self.tables.insert(table_name.clone(), table);
        let serialized = bincode::serialize(&self.tables.get(&table_name).unwrap())?;
        self.db.insert(table_name.as_bytes(), serialized)?;
        Ok(())
    }

    pub fn insert(
        &self,
        table_name: &str,
        data: HashMap<String, String>,
    ) -> Result<(), Box<dyn Error>> {
        let table = self.tables.get(table_name).ok_or("Table not found.")?;
        let mut serialized_data = Vec::new();

        for column in &table.columns {
            let value = data.get(&column.name).ok_or("Column not found in data")?;
            serialized_data.extend_from_slice(value.as_bytes());
        }

        table.columns.iter();

        Ok(())
    }
}
