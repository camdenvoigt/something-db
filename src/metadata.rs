use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
enum FieldError {
    #[error("Bad Field Type: `{0}`")]
    BadType(String),
}

#[derive(Debug)]
pub enum FieldType {
    Int(u64),
    Char(u64),
}

impl FieldType {
    fn parse(ft: String) -> Result<FieldType, FieldError> {
        if ft.as_str().starts_with("int") {
            return Ok(FieldType::Int(64));
        }

        if ft.as_str().starts_with("char") {
            return Ok(FieldType::Char(64));
        }

        Err(FieldError::BadType(ft.clone()))
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct FieldJson {
    name: String,
    field_type: String,
}

#[derive(Debug)]
pub struct Field {
    name: String,
    field_type: FieldType,
}

impl Field {
    fn from_json(json_field: FieldJson) -> Self {
        Field {
            name: json_field.name.clone(),
            field_type: FieldType::parse(json_field.field_type).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TableJson {
    table_name: String,
    fields: Vec<FieldJson>,
}

#[derive(Debug)]
pub struct Table {
    table_name: String,
    fields: Vec<Field>,
}

impl Table {
    fn from_json(json_table: TableJson) -> Self {
        Table {
            table_name: json_table.table_name.clone(),
            fields: json_table
                .fields
                .iter()
                .map(|json_field| Field::from_json(json_field.clone()))
                .collect(),
        }
    }
}

pub fn parse_table(file_path: &str) -> anyhow::Result<Table> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);

    let json_table = serde_json::from_reader(reader)?;

    Ok(Table::from_json(json_table))
}
