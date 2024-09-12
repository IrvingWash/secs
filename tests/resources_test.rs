use secs::World;

#[test]
fn create_ang_get_resource_immutably() {
    let mut world = World::new();

    world.add_resource(FPSResource(60));

    let fps = world.get_resource::<FPSResource>().unwrap();

    assert_eq!(fps.0, 60);
}

struct FPSResource(pub u32);
