mod db;
mod metadata;

use crate::{db::InsertData, metadata::Table};

fn main() -> anyhow::Result<()> {
    let table = Table::new("./test_files/person_table.json");
    println!("{:?}", table);

    let table_metadata = table.get_table_metadata();
    println!("{:?}", table_metadata);
    table_metadata.save()?;

    let insert_data = InsertData::parse_data("./test_files/person_data.json")?;
    println!("{:?}", insert_data);
    insert_data.save()?;

    Ok(())
}
