use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;

const DEFAULT_BLOCK_SIZE: usize = 4096;

struct DataProcessor {
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
            let mut f = OpenOptions::new().append(true).open("./db")?;
            f.write_all(buf)?;
            self.buf = [0; DEFAULT_BLOCK_SIZE];
            self.index = 0;
            return Ok(self);
        }

        Ok(self)
    }
}

fn main() {
    let db = DataProcessor::new();
    let db = db.save(&[42; DEFAULT_BLOCK_SIZE]).unwrap();
    db.save(&[36; DEFAULT_BLOCK_SIZE]).unwrap();
}
