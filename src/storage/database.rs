use super::saveable::Saveable;
use std::fs::File;
use std::io::Write;

pub fn save(data: impl Saveable) -> std::io::Result<()> {
    let mut file = File::create("db.txt")?;
    file.write_all(data.get_data())?;
    Ok(())
}
