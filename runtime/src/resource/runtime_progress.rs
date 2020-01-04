
#[derive(Debug, Clone, Serialize)]
pub struct RuntimeProgress {
    pub current_segment: u8,
    pub segment_count: u8,
    pub segment_times: Vec<f64>,
}

impl Default for RuntimeProgress {
    fn default() -> Self {
        RuntimeProgress {
            current_segment: 1u8,
            segment_count: 0u8,
            segment_times: vec![],
        }
    }
}

impl RuntimeProgress {
    pub fn new(segment_count: u8) -> Self {
        RuntimeProgress {
            current_segment: 1u8,
            segment_count,
            // +1 to take into account last segment to end zone
            segment_times: vec![0.0; (segment_count) as usize],
        }
    }
}
