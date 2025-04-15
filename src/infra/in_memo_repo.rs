use crate::error::ErrDB;

pub struct InMemoryRepo<T> {
    repo: Vec<T>,
}

impl<T> InMemoryRepo<T> {
    pub async fn new() -> Self {
        Self {
            repo: Vec::<T>::new(),
        }
    }
}

// #[async_trait]
impl<T> InMemoryRepo<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    pub async fn insert_data(&mut self, data: &T) -> Result<(), ErrDB> {
        Ok(self.repo.push(data.clone()))
    }

    pub async fn remove_data(&mut self, data: &T) -> Result<(), ErrDB> {
        if let Some(pos) = self.repo.iter().position(|x| x == data) {
            self.repo.remove(pos);
        } else {
            return Err(ErrDB::Unreachable);
        }
        Ok(())
    }
    pub async fn list(&self) -> Result<Vec<T>, ErrDB> {
        let mut vec = vec![];
        for (_i, element) in self.repo.iter().enumerate() {
            vec.push(element.clone());
        }
        Ok(vec)
    }
    pub async fn is_exist(&self, data: &T) -> Result<bool, ErrDB> {
        if self.repo.iter().any(|x| x == data) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
