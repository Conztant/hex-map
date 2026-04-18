use hex_grid::{CubeCoord, HexGrid};

use crate::core::error::HexMapError;
use crate::core::tile::TileData;
use crate::pipeline::operation::GeneratorOperation;
use crate::util::rng::SeededRng;

pub struct VoronoiPartitionOp {
    region_count: usize,
}

impl VoronoiPartitionOp {
    pub fn new(region_count: usize) -> Self {
        Self { region_count }
    }

    fn choose_centers(&self, coords: &mut [CubeCoord], rng: &mut SeededRng) -> Vec<CubeCoord> {
        rng.shuffle(coords);
        coords[..self.region_count].to_vec()
    }
}

impl GeneratorOperation for VoronoiPartitionOp {
    fn name(&self) -> &'static str {
        "voronoi_partition"
    }

    fn apply(&self, map: &mut HexGrid<TileData>, rng: &mut SeededRng) -> Result<(), HexMapError> {
        let mut coords: Vec<CubeCoord> = map.iter().map(|(coord, _)| coord).collect();

        if self.region_count == 0 || self.region_count > coords.len() {
            return Err(HexMapError::InvalidVoronoiRegionCount);
        }

        let centers = self.choose_centers(&mut coords, rng);
        let all_coords: Vec<CubeCoord> = map.iter().map(|(coord, _)| coord).collect();

        for coord in all_coords {
            let mut best_id = 0usize;
            let mut best_distance = i32::MAX;

            for (idx, center) in centers.iter().enumerate() {
                let distance = map.wrapped_distance(coord, *center);
                if distance < best_distance || (distance == best_distance && idx < best_id) {
                    best_distance = distance;
                    best_id = idx;
                }
            }

            if let Some(tile) = map.get_mut(coord) {
                tile.cell_id = Some(best_id as u32);
            }
        }

        Ok(())
    }
}
