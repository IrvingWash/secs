use secs::World;

struct FPSResource(pub u32);

#[test]
fn create_ang_get_resource_immutably() {
    let mut world = World::new();

    world.add_resource(FPSResource(60));

    let fps = world.get_resource::<FPSResource>().unwrap();

    assert_eq!(fps.0, 60);
}

#[test]
fn get_resources_mutably() {
    let mut world = World::new();

    world.add_resource(FPSResource(60));

    let fps: &mut FPSResource = world.get_resource_mut::<FPSResource>().unwrap();

    fps.0 += 60;

    assert_eq!(world.get_resource::<FPSResource>().unwrap().0, 120);
}
