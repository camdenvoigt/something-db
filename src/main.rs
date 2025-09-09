mod storage;
use crate::storage::database;
use crate::storage::saveable::Saveable;

fn main() {
    let data = TextData {
        data: String::from("Hello world!"),
    };
    let _ = database::save(data);
}

struct TextData {
    data: String,
}

impl Saveable for TextData {
    fn get_data(&self) -> &[u8] {
        self.data.as_bytes()
    }
}
