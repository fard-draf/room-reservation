use std::fmt::Debug;

use crate::repository::DBRepository;

pub struct InMemoryRepo<T> {
    repo: Vec<T>,
}

impl<T> InMemoryRepo<T> {
    pub fn new() -> Self {
        Self {
            repo: Vec::<T>::new(),
        }
    }
}

impl<T: PartialEq + Debug + Clone> DBRepository<T> for InMemoryRepo<T> {
    fn insert_data(&mut self, data: &T) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.repo.push(data.clone()))
    }

    fn remove_data(&mut self, data: &T) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(pos) = self.repo.iter().position(|x| x == data) {
            self.repo.remove(pos);
        } else {
            Err("No correspondance.")?;
        }
        Ok(())
    }
    fn list(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let mut vec = vec![];
        for (_i, element) in self.repo.iter().enumerate() {
            vec.push(element.clone());
        }
        Ok(vec)
    }
    fn is_empty(&self, data: &T) -> Result<bool, Box<dyn std::error::Error>> {
        if self.repo.iter().any(|x| x == data) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
