#[macro_use]
extern crate serde;
extern crate uuid;
extern crate cgmath;

use cgmath::{Vector3, Quaternion};
use uuid::Uuid;
use std::collections::VecDeque;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct PlayerStats {
	pub jumps: u32,
	pub strafes: u32,
	pub playcount: u32,
}

/// Number for keys is amount [0,1] (controller)
/// Actions don't have a number or have exact values (rotation angle)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HoppinInput {
	Forward(f32),
	Backward(f32),
	Right(f32),
	Left(f32),
	Rotate(f32,f32),
	Jump,
}

/// Replay data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Replay {
	/// Time -> Input (type?)
	pub inputs: VecDeque<(f64,HoppinInput)>,
	/// To accomodate for non-determinism and frame-rate dependent acceleration (not using an integral, so polling rate differs)
	/// Time -> position
	/// Checked by score validator and if its in the margin it will correct the server player to the sync point.
	/// If its over the margin the score will be rejected.
	pub transform_sync: VecDeque<(f64, Vector3<f32>, Quaternion<f32>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MapCategory {
    Speed,
    Technical,
    Gimmick,
    Uphill,
    Downhill,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MapDifficulty {
    Easy,
    Normal,
    Hard,
    Insane,
    Extreme,
    Master,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapInfo {
    pub id: Uuid,
    pub name: String,
    pub mapper: String,
    pub categories: Vec<MapCategory>,
    pub difficulty: MapDifficulty,
    pub tags: Vec<String>,
    pub segment_count: u8,
}