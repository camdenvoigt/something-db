use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{BufReader, Bytes};
use thiserror::Error;

use crate::metadata::{FieldType, TableMetadata};

pub const DEFAULT_BLOCK_SIZE: usize = 4096;

pub struct DataProcessor {
    buf: [u8; DEFAULT_BLOCK_SIZE],
    index: u8,
}

impl DataProcessor {
    pub fn new() -> Self {
        DataProcessor {
            buf: [0; DEFAULT_BLOCK_SIZE],
            index: 0,
        }
    }

    //TODO: Make this more robust. Not handling even simple edge cases.
    pub fn save(mut self, buf: &[u8]) -> Result<DataProcessor> {
        // write to block buffer
        for i in (self.index as usize)..(self.index as usize + buf.len()) {
            self.buf[i] = buf[i - self.index as usize];
        }

        // if block buffer full write to disk
        if self.index >= DEFAULT_BLOCK_SIZE as u8 {
            let mut f = OpenOptions::new().append(true).open("./db/db")?;
            f.write_all(buf)?;
            self.buf = [0; DEFAULT_BLOCK_SIZE];
            self.index = 0;
            return Ok(self);
        }

        Ok(self)
    }
}

#[derive(Debug, Error)]
enum InsertDataError {
    #[error("Value was too large for field")]
    ValueTooLarge,
    #[error("value was the wrong type for field")]
    MismatchedTypes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertData {
    table_name: String,
    data: Vec<serde_json::Value>,
}

impl InsertData {
    fn extract_string(v: serde_json::Value, size: u64) -> anyhow::Result<String, InsertDataError> {
        if let serde_json::Value::String(val) = v {
            if val.len() > size as usize {
                Err(InsertDataError::ValueTooLarge)
            } else {
                Ok(val)
            }
        } else {
            Err(InsertDataError::MismatchedTypes)
        }
    }

    fn extract_int(v: serde_json::Value, size: u64) -> anyhow::Result<u64, InsertDataError> {
        if let serde_json::Value::Number(val) = v {
            if !val.is_u64() {
                Err(InsertDataError::ValueTooLarge)
            } else {
                Ok(val.as_u64().unwrap())
            }
        } else {
            Err(InsertDataError::MismatchedTypes)
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let table_metadata = TableMetadata::get(&self.table_name)?;
        let file_path = format!("./db/{}_table", self.table_name);
        let mut f: File;
        if std::fs::exists(&file_path)? {
            f = OpenOptions::new().append(true).open(file_path)?;
        } else {
            f = File::create(file_path)?;
        }

        for value in &self.data {
            let mut row: Vec<u8> = vec![];
            for field in &table_metadata.fields {
                let val = value[&field.name].clone();
                match field.field_type {
                    FieldType::Char(n) => {
                        let val = InsertData::extract_string(val, n)?;
                        for b in val.bytes() {
                            row.push(b);
                        }
                    }
                    FieldType::Int(n) => {
                        let val = InsertData::extract_int(val, n)?;
                        for b in val.to_le_bytes() {
                            row.push(b);
                        }
                    }
                }
            }

            f.write_all(row.as_slice())?;
        }

        Ok(())
    }

    pub fn parse_data(file_path: &str) -> anyhow::Result<Self> {
        let f = File::open(file_path)?;
        let reader = BufReader::new(f);
        let insert_data = serde_json::from_reader(reader)?;
        Ok(insert_data)
    }
}
