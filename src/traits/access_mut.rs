pub trait AccessMut{
    fn get(&self) -> &Self;
    fn set(&self) -> &Self;
}