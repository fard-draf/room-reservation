use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

#[derive(Debug)]
pub struct InMemoryRepo<T> {
    pub repo: Mutex<HashMap<Uuid, T>>,
}

impl<T> InMemoryRepo<T> {
    pub async fn new() -> Self {
        Self {
            repo: Mutex::new(HashMap::<Uuid, T>::new()),
        }
    }
}
