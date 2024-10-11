use core::fmt;
use prettytable::{Cell, Row as PrettyRow, Table as PrettyTable};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::data_types::{Column, Value};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row {
    pub values: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    columns: Vec<Column>,
    data: BTreeMap<usize, Row>,
    indexes: BTreeMap<String, BTreeMap<Value, Vec<usize>>>,
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Table {
            columns,
            data: BTreeMap::new(),
            indexes: BTreeMap::new(),
        }
    }

    pub fn insert_row(&mut self, row_id: usize, values: Vec<Value>) -> Result<(), String> {
        if values.len() != self.columns.len() {
            return Err("Number of values doesn't match number of columns".to_string());
        }

        for (value, column) in values.iter().zip(self.columns.iter()) {
            if !value.matches_type(&column.data_type) {
                return Err(format!("Type mismatch for column '{}'", column.name));
            }
        }

        let row = Row {
            values: values.clone(),
        };
        self.data.insert(row_id, row);

        // Update indexes
        for (i, (column, value)) in self.columns.iter().zip(values.iter()).enumerate() {
            self.indexes
                .entry(column.name.clone())
                .or_insert_with(BTreeMap::new)
                .entry(value.clone())
                .or_insert_with(Vec::new)
                .push(row_id);
        }

        Ok(())
    }

    pub fn select(&self, columns: &[String]) -> Result<Vec<Row>, String> {
        let column_indices: Vec<usize> = columns
            .iter()
            .map(|col| self.columns.iter().position(|c| c.name == *col))
            .collect::<Option<Vec<usize>>>()
            .ok_or_else(|| "One or more columns not found".to_string())?;

        Ok(self
            .data
            .values()
            .map(|row| Row {
                values: column_indices
                    .iter()
                    .map(|&i| row.values[i].clone())
                    .collect(),
            })
            .collect())
    }

    pub fn get_columns(&self) -> &Vec<Column> {
        &self.columns
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pretty_table = PrettyTable::new();

        // Add header row
        let header = PrettyRow::new(
            self.columns
                .iter()
                .map(|col| Cell::new(&col.name))
                .collect(),
        );
        pretty_table.add_row(header);

        // Add data rows
        for row in self.data.values() {
            let pretty_row = PrettyRow::new(
                row.values
                    .iter()
                    .map(|value| Cell::new(&value_to_string(value)))
                    .collect(),
            );
            pretty_table.add_row(pretty_row);
        }

        write!(f, "{}", pretty_table)
    }
}

// Helper function to convert Value to String
fn value_to_string(value: &Value) -> String {
    match value {
        Value::Integer(i) => i.to_string(),
        Value::Text(s) => s.clone(),
        // Add more variants as needed
    }
}
