use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::io::Write;
use std::{fs::File, io::BufReader};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde[tag = "type", content = "size"]]
pub enum FieldType {
    Int(u64),
    Char(u64),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    table_name: String,
    fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableMetadata {
    pub table_name: String,
    pub num_fields: usize,
    pub row_length: u64,
    pub fields: Vec<Field>,
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
                    FieldType::Int(v) => v / 8,
                };
                acc += value;
                acc
            }),
        }
    }
}

impl TableMetadata {
    pub fn save(&self) -> Result<()> {
        let file_name = format!("./db/{}_metadata.json", self.table_name);
        let json_obj = to_string(self)?;
        let mut f = File::create(file_name)?;
        f.write_all(json_obj.as_bytes())?;
        Ok(())
    }

    pub fn get(table_name: &String) -> Result<Self> {
        let file_path = format!("./db/{}_metadata.json", table_name);
        let f = File::open(file_path)?;
        let reader = BufReader::new(f);
        let table_metadata = serde_json::from_reader(reader)?;

        Ok(table_metadata)
    }
}
