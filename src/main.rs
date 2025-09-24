mod db;
mod metadata;

use crate::metadata::Table;

fn main() {
    let table = Table::new("./test_files/person_table.json");
    println!("{:?}", table);
    println!("{:?}", table.get_table_metadata());
}
