use std::any::Any;

use resources::Resources;

mod resources;

#[derive(Debug, Default)]
/// The storage for entities, components and resources.
///
/// ## Example
/// ```
/// # use secs::World;
/// let mut world = World::new();
/// ```
pub struct World {
    resources: Resources,
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: Resources::default(),
        }
    }

    /// Add a new resource of any type
    ///
    /// ## Example
    /// ```
    /// # use secs::World;
    /// struct Score(pub u32);
    /// let mut world = World::new();
    ///
    /// world.add_resource(Score(100));
    /// ```
    pub fn add_resource(&mut self, resource: impl Any) {
        self.resources.add(resource);
    }

    /// Query for a resource and get an immutable reference to it.
    ///
    /// ## Example
    /// ```
    /// # use secs::World;
    /// struct Score(u32);
    ///
    /// let mut world = World::new();
    /// world.add_resource(Score(100));
    ///
    /// let score: Option<&Score> = world.get_resource::<Score>();
    /// assert_eq!(score.unwrap().0, 100);
    /// ```
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /// Query for a resource and get a mutable reference to it.
    ///
    /// ## Example
    /// ```
    /// # use secs::World;
    /// struct Score(u32);
    ///
    /// let mut world = World::new();
    /// world.add_resource(Score(100));
    ///
    /// let score: Option<&mut Score> = world.get_resource_mut::<Score>();
    ///
    /// score.unwrap().0 += 5;
    ///
    /// assert_eq!(world.get_resource::<Score>().unwrap().0, 105);
    /// ```
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }
}
