use std::{any::Any, cell::RefCell, rc::Rc};

use secs::World;

struct Position(pub f64, pub f64);
struct Rotation(pub f64);

#[test]
fn create_entity() -> Result<(), String> {
    let mut world = World::new();

    world.register_component::<Position>();
    world.register_component::<Rotation>();

    world
        .create_entity()
        .with_component(Position(69., 420.))?
        .with_component(Rotation(90.))?;

    Ok(())
}

#[test]
fn query_for_entities() -> Result<(), String> {
    let mut world = World::new();

    world.register_component::<Position>();
    world.register_component::<Rotation>();

    world
        .create_entity()
        .with_component(Position(69., 420.))?
        .with_component(Rotation(90.))?;

    world.create_entity().with_component(Position(1., 2.))?;

    world.create_entity().with_component(Rotation(0.))?;

    world
        .create_entity()
        .with_component(Position(1337., 303.))?
        .with_component(Rotation(45.))?;

    let query = world
        .query()
        .with_component::<Position>()
        .with_component::<Rotation>()
        .run();

    let positions: &Vec<Rc<RefCell<dyn Any>>> = &query[0];
    let rotations: &Vec<Rc<RefCell<dyn Any>>> = &query[1];

    assert_eq!(positions.len(), rotations.len());

    assert_eq!(positions.len(), 2);
    assert_eq!(rotations.len(), 2);

    let binding = positions[0].borrow();
    let first_position = binding.downcast_ref::<Position>().unwrap();
    let binding = rotations[0].borrow();
    let first_rotation = binding.downcast_ref::<Rotation>().unwrap();

    assert_eq!(first_position.0, 69.);
    assert_eq!(first_rotation.0, 90.);

    let binding = positions[1].borrow();
    let second_position = binding.downcast_ref::<Position>().unwrap();
    let binding = rotations[1].borrow();
    let second_rotation = binding.downcast_ref::<Rotation>();

    assert_eq!(second_position.0, 1.);
    assert!(second_rotation.is_none());

    Ok(())
}
