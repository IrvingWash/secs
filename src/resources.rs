use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Debug, Default)]
pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn add(&mut self, resource: impl Any) {
        self.data.insert(resource.type_id(), Box::new(resource));
    }

    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        if let Some(data) = self.data.get(&TypeId::of::<T>()) {
            return data.downcast_ref();
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct WorldWidth(pub f32);

    #[test]
    fn add_resource() {
        let mut resources = Resources::default();

        let world_width = WorldWidth(420.);

        assert_eq!(resources.data.len(), 0);

        resources.add(world_width);

        assert_eq!(resources.data.len(), 1);
    }

    #[test]
    fn get_resource() {
        let mut resources = Resources::default();
        let world_width = WorldWidth(420.);

        resources.add(world_width);

        let extracted_world_width = resources.get_ref::<WorldWidth>().unwrap();

        assert_eq!(extracted_world_width.0, 420.);
    }
}
