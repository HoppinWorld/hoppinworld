
use amethyst::ecs::{System, ReadStorage, WriteStorage, Join};
use crate::component::{PlayerFeetTag, PlayerTag};
use amethyst::core::Transform;
use amethyst::core::math::Vector3;
use amethyst_extra::nphysics_ecs::*;
use amethyst_extra::nphysics_ecs::bodies::*;

pub struct PlayerFeetSync;

impl<'a> System<'a> for PlayerFeetSync {
    type SystemData = (
        ReadStorage<'a, PlayerFeetTag>,
        ReadStorage<'a, PlayerTag>,
        WriteStorage<'a, Transform>,
        WriteRigidBodies<'a, f32>,
    );

    fn run(&mut self, (player_feets, players, mut transforms, mut rigid_bodies): Self::SystemData) {
        // Player in scene
        if let Some((player_position, vel)) = (&players, &transforms, &rigid_bodies).join().next().map(|e| {
                (e.1.translation().clone(), e.2.velocity().clone())
            }) {
            // TODO: Replace -0.4 by player half_height
            *(&player_feets, &mut transforms).join().next().expect("No player feet but player is in scene.").1.translation_mut() = Vector3::new(player_position.x, player_position.y - 0.4, player_position.z);
            /*if let DynamicBody::RigidBody(ref mut rb) = *(&player_feets, &mut rigid_bodies).join().next().expect("No player feet but player is in scene.").1 {
                rb.velocity = vel;
            }*/
        }
    }
}
