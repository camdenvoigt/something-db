use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
enum FieldError {
    #[error("Bad Field Type: `{0}`")]
    BadType(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde[tag = "type", content = "size"]]
pub enum FieldType {
    Int(u64),
    Char(u64),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    name: String,
    field_type: FieldType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    table_name: String,
    fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableMetadata {
    table_name: String,
    num_fields: usize,
    row_length: u64,
    fields: Vec<Field>,
}

impl Table {
    pub fn new(file_path: &str) -> Self {
        Table::parse_table(file_path).unwrap()
    }

    pub fn parse_table(file_path: &str) -> anyhow::Result<Table> {
        let f = File::open(file_path)?;
        let reader = BufReader::new(f);
        let table = serde_json::from_reader(reader)?;

        Ok(table)
    }

    pub fn get_table_metadata(&self) -> TableMetadata {
        TableMetadata {
            table_name: self.table_name.clone(),
            fields: self.fields.clone(),
            num_fields: self.fields.len(),
            row_length: self.fields.iter().fold(0, |mut acc, field| {
                let value = match field.field_type {
                    FieldType::Char(v) => v,
                    FieldType::Int(v) => v,
                };
                acc += value;
                acc
            }),
        }
    }
}
