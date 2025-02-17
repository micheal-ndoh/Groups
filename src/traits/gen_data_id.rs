pub trait GenDataId<T: Sized> {
    fn set_id(&mut self, id: u32);
    fn get_id(&self) -> T;
}