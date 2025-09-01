pub trait Saveable {
    fn get_data(&self) -> &[u8];
}
