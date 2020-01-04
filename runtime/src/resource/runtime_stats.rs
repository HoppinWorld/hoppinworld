use std::collections::VecDeque;

#[derive(Default, Debug, Clone, Serialize, Deserialize, new)]
pub struct RuntimeStats {
	pub jumps: u32,
	pub strafes: u32,
	pub jump_timings: VecDeque<(f64, f32)>,
}