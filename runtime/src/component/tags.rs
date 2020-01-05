use amethyst::ecs::*;

pub struct PlayerFeetTag;

impl Component for PlayerFeetTag {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Serialize, Deserialize)]
pub struct PlayerTag;

impl Component for PlayerTag {
    type Storage = DenseVecStorage<Self>;
}
