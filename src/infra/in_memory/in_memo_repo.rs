use std::{collections::HashMap, sync::Mutex};

pub struct InMemoryRepo<T> {
    pub repo: Mutex<HashMap<i32, T>>,
}

impl<T> InMemoryRepo<T> {
    pub async fn new() -> Self {
        Self {
            repo: Mutex::new(HashMap::<i32, T>::new()),
        }
    }
}
