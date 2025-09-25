mod db;
mod metadata;

use crate::metadata::Table;

fn main() -> anyhow::Result<()> {
    let table = Table::new("./test_files/person_table.json");
    println!("{:?}", table);
    let table_metadata = table.get_table_metadata();
    println!("{:?}", table_metadata);
    table_metadata.save()?;
    Ok(())
}
