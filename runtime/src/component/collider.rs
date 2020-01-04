use amethyst::ecs::*;
use half_matrix::HalfMatrix;
use amethyst_extra::nphysics_ecs::ncollide::pipeline::object::CollisionGroups;

lazy_static! {
    static ref COLLISION_MATRIX: HalfMatrix = generate_collision_matrix();
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum ObjectType {
    Scene, // 0
    StartZone, // 1
    EndZone, // 2
    KillZone, // 3
    Player, // 4
    PlayerFeet, // 5
    Dynamic, // 6
    Ignore, // 7
    SegmentZone(u8), // 8
}

impl Component for ObjectType {
    type Storage = DenseVecStorage<Self>;
}

impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::Scene
    }
}

impl From<ObjectType> for u32 {
    fn from(o: ObjectType) -> u32 {
        match o {
            ObjectType::Scene => 0,
            ObjectType::StartZone => 1,
            ObjectType::EndZone => 2,
            ObjectType::KillZone => 3,
            ObjectType::Player => 4,
            ObjectType::PlayerFeet => 5,
            ObjectType::Dynamic => 6,
            ObjectType::Ignore => 7,
            ObjectType::SegmentZone(_) => 8,
        }
    }
}

impl From<ObjectType> for CollisionGroups {
    fn from(o: ObjectType) -> CollisionGroups {
        CollisionGroups::new().with_membership(&[u32::from(o) as usize]).with_whitelist(whitelist_for_object(o).as_slice())
   }
}

pub fn generate_collision_matrix() -> HalfMatrix {
    let mut m = HalfMatrix::new(9);
    m.enable(4,0); // Player Scene
    m.enable(4,1);
    m.enable(4,2);
    m.enable(4,3);
    m.enable(4,6);
    m.enable(4,8);
    m.enable(5,0); // PlayerFeet Scene
    m.enable(5,6);
    m
}

pub fn whitelist_for_object(o: ObjectType) -> Vec<usize> {
    let mut res = vec![];
    let id = u32::from(o);
    for i in 0..COLLISION_MATRIX.size() {
        if i != id {
            if COLLISION_MATRIX.contains(i, id) {
                res.push(i as usize);
            }
        }
    }
    res
}
