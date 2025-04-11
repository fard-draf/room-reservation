use std::error::Error;

pub trait DBRepository<T> {
    fn insert_data(&mut self, data: &T) -> Result<(), Box<dyn Error>>;
    fn remove_data(&mut self, data: &T) -> Result<(), Box<dyn Error>>;
    fn list(&self) -> Result<Vec<T>, Box<dyn Error>>;
    fn is_empty(&self, data: &T) -> Result<bool, Box<dyn Error>>;
}

