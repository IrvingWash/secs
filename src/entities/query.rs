use std::any::Any;

use super::Rra;

pub struct Query {}

impl Query {
    pub fn new() -> Self {
        Self {}
    }

    pub fn with_component<T: Any>(&mut self) -> &mut Self {
        self
    }

    pub fn run(&self) -> Vec<Vec<Rra>> {
        Vec::new()
    }
}
