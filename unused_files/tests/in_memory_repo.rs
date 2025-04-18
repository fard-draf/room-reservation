#[derive(Debug)]
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
