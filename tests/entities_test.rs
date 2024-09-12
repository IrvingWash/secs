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
