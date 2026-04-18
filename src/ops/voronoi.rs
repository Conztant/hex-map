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

        let centers = super::choose_centers(&mut coords, self.region_count, rng);

        for &coord in &coords {
            let id = super::nearest_center_id(coord, &centers, map) as u32;
            if let Some(tile) = map.get_mut(coord) {
                tile.cell_id = Some(id);
            }
        }

        Ok(())
    }
}
