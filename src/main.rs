mod db;
mod metadata;

fn main() {
    let table = metadata::parse_table("./test_files/person_table.json");
    println!("{:?}", table.unwrap());
}
