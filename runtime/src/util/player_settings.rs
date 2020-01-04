use amethyst_extra::{Grounded, BhopMovement3D, GroundFriction3D};
use amethyst_extra::nphysics_ecs::*;

#[derive(Debug, new, Clone, Serialize, Deserialize)]
pub struct PlayerSettings {
    pub grounded: Grounded,
    pub movement: BhopMovement3D,
    pub ground_friction: GroundFriction3D,
    //pub shape: ShapeHandle<f32>,
    //pub physical_entity: PhysicsBody<f32>,
    pub mass: f32,
    pub gravity: f32,
    pub jump_velocity: f32,
}
