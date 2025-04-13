use crate::error::ErrDB;

pub trait DBRepository<T> {
    fn insert_data(&mut self, data: &T) -> Result<(), ErrDB>;
    fn remove_data(&mut self, data: &T) -> Result<(), ErrDB>;
    fn list(&self) -> Result<Vec<T>, ErrDB>;
    fn is_empty(&self, data: &T) -> Result<bool, ErrDB>;
}
