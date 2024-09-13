use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

pub mod query;

type Rra = Rc<RefCell<dyn Any>>;

const COMPONENT_NOT_REGISTERED_MESSAGE: &str = "Failed to add a component, has it been registered?";

#[derive(Debug, Default)]
pub struct Entities {
    components: HashMap<TypeId, Vec<Option<Rra>>>,
}

impl Entities {
    pub fn register_component<T: Any>(&mut self) {
        self.components.insert(TypeId::of::<T>(), Vec::new());
    }

    pub fn create_entity(&mut self) -> &mut Self {
        // TODO: this works well until the first removal of an entity
        // We may have empty entities in the middle of the vectors.
        self.components
            .iter_mut()
            .for_each(|(_types, components)| components.push(None));

        self
    }

    pub fn with_component(&mut self, component: impl Any) -> Result<&mut Self, &str> {
        if let Some(components) = self.components.get_mut(&component.type_id()) {
            let last = components
                .last_mut()
                .ok_or(COMPONENT_NOT_REGISTERED_MESSAGE)?;

            *last = Some(Rc::new(RefCell::new(component)));

            return Ok(self);
        }

        Err(COMPONENT_NOT_REGISTERED_MESSAGE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Health(pub u64);
    struct Mana(pub i64);

    #[test]
    fn register_entity() {
        let mut entities = Entities::default();

        entities.register_component::<Health>();

        assert_eq!(
            entities
                .components
                .get(&TypeId::of::<Health>())
                .unwrap()
                .len(),
            0
        );
    }

    #[test]
    fn create_entity() {
        let mut entities = Entities::default();

        entities.register_component::<Health>();
        entities.register_component::<Mana>();

        entities.create_entity();

        let healths = entities.components.get(&TypeId::of::<Health>()).unwrap();
        let manas = entities.components.get(&TypeId::of::<Mana>()).unwrap();

        assert_eq!(healths.len(), 1);
        assert_eq!(manas.len(), 1);
    }

    #[test]
    fn with_component() -> Result<(), String> {
        let mut entities = Entities::default();

        entities.register_component::<Health>();
        entities.register_component::<Mana>();

        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Mana(-5))?;

        let binding = entities.components.get(&TypeId::of::<Health>()).unwrap()[0]
            .as_ref()
            .unwrap()
            .borrow();
        let health = binding.downcast_ref::<Health>().unwrap();

        let binding = entities.components.get(&TypeId::of::<Mana>()).unwrap()[0]
            .as_ref()
            .unwrap()
            .borrow();
        let mana = binding.downcast_ref::<Mana>().unwrap();

        assert_eq!(health.0, 100);
        assert_eq!(mana.0, -5);

        Ok(())
    }
}
