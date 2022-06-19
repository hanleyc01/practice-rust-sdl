use specs::prelude::*;
use crate::components::*;

/// Struct system abstraction for our physics, note, as in `fn run`, this applies to anything
/// which has both a velocity and a position.
pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    /// Running abstraction utilizing our data storage; note that this abstraction can apply
    /// to anything which has both a position and a velocity, thus allowing an update
    fn run(&mut self, mut data: Self::SystemData) {
        // Todo!; this code can be made more idiomatic using more pattern matching
        // Lookup "rust irrefutable patterns", and refactor them
        use self::Direction::*;
        (&mut data.0, &data.1).par_join()
            .for_each(|(pos, vel)| match vel.direction {
                Left => {
                    pos.0 = pos.0.offset(-vel.speed, 0);
                },
                Right => {
                    pos.0 = pos.0.offset(vel.speed, 0);
                },
                Up => {
                    pos.0 = pos.0.offset(0, -vel.speed);
                },
                Down => {
                    pos.0 = pos.0.offset(0, vel.speed);
                },
            });
    }       
}


