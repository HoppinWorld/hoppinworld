use amethyst::ecs::Entity;
use hoppinworld_data::MapInfo;

#[derive(Debug, Clone)]
pub struct RuntimeMap {
    pub start_zone: Entity,
    pub end_zone: Entity,
    pub segment_zones: Vec<(u8, Entity)>,
}

#[derive(Default)]
pub struct RuntimeMapBuilder {
    pub start_zone: Option<Entity>,
    pub end_zone: Option<Entity>,
    pub segment_zones: Vec<(u8, Entity)>,
}

impl RuntimeMapBuilder {
    pub fn start_zone(mut self, entity: Entity) -> Self {
        self.start_zone = Some(entity);
        self
    }

    pub fn end_zone(mut self, entity: Entity) -> Self {
        self.end_zone = Some(entity);
        self
    }

    pub fn build(mut self, map_info: &MapInfo) -> Result<RuntimeMap, String> {
        let start_zone = self.start_zone.ok_or("StartZone is not present in map.")?;
        let end_zone = self.end_zone.ok_or("EndZone is not present in map.")?;

        order_segment_zones(&mut self.segment_zones);
        validate_segment_zones(&self.segment_zones, map_info)?;
        Ok(RuntimeMap {
            start_zone,
            end_zone,
            segment_zones: self.segment_zones,
        })
    }
}

pub fn order_segment_zones(segment_zones: &mut Vec<(u8, Entity)>) {
    segment_zones.sort_unstable_by(|a, b| a.0.cmp(&b.0));
}

/// Checks if the number of segments matches the number of segments indicated in the map info file.
/// Checks if the segment zones are in order and continuous and start at 1 and don't go over 254 and are not duplicated.
/// Expects the segment zones to be ordered by id.
pub fn validate_segment_zones(
    segment_zones: &Vec<(u8, Entity)>,
    map_info: &MapInfo,
) -> Result<(), String> {
    if segment_zones.len() > 254 {
        return Err(format!(
            "Failed to load map: Too many segment zones (max 254)."
        ));
    }

    if (segment_zones.len() + 1) as u8 != map_info.segment_count {
        return Err(format!(
            "Failed to load map:\nThe segment zones count in the gltf (glb) map file + 1
			doesn't match the segment_count value of the map info file (.hop)\n
			glTF: {} + 1, map info: {}",
            segment_zones.len(),
            map_info.segment_count
        ));
    }

    let mut last = 0u8;
    for seg_id in segment_zones.iter().map(|t| t.0) {
        // Duplicate id.
        if seg_id == last {
            return Err(format!(
                "Failed to load map: Two segment zones have the same id: {}",
                seg_id
            ));
        }
        // Non-continuous id distribution.
        if seg_id != last + 1u8 {
            return Err(format!(
                "Failed to load map: There is a gap in the segment zone id. Jumped from {} to {}",
                last, seg_id
            ));
        }
        last = seg_id;
    }

    // Good to go!
    Ok(())
}
