use std::any::Any;

use resources::Resources;

mod resources;

#[derive(Debug, Default)]
pub struct World {
    resources: Resources,
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: Resources::default(),
        }
    }

    pub fn add_resource(&mut self, resource: impl Any) {
        self.resources.add(resource);
    }

    pub fn get_resource<T: Any>(&mut self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }
}
