use hex_grid::HexGrid;

use crate::core::error::HexMapError;
use crate::core::tile::{TerrainKind, TileData};
use crate::pipeline::operation::GeneratorOperation;
use crate::util::rng::SeededRng;

pub struct LandRaiseSinkOp {
    land_ratio_percent: u32,
    max_raise: i32,
    max_sink: i32,
}

impl LandRaiseSinkOp {
    pub fn new(land_ratio_percent: u32, max_raise: i32, max_sink: i32) -> Self {
        Self {
            land_ratio_percent: land_ratio_percent.min(100),
            max_raise: max_raise.max(0),
            max_sink: max_sink.max(0),
        }
    }
}

impl GeneratorOperation for LandRaiseSinkOp {
    fn name(&self) -> &'static str {
        "land_raise_sink"
    }

    fn apply(&self, map: &mut HexGrid<TileData>, rng: &mut SeededRng) -> Result<(), HexMapError> {
        // Collect unique cell IDs (sorted for determinism), then roll land/water once per cell.
        let unique_ids: Vec<u32> = {
            let mut ids: Vec<u32> = map.iter().filter_map(|(_, tile)| tile.cell_id).collect();
            ids.sort_unstable();
            ids.dedup();
            ids
        };

        // Index cell decisions by cell_id (IDs are 0-based from VoronoiPartitionOp).
        let max_id = unique_ids.last().copied().unwrap_or(0) as usize;
        let mut cell_is_land = vec![false; max_id + 1];
        for &id in &unique_ids {
            cell_is_land[id as usize] = rng.next_bool_ratio(self.land_ratio_percent, 100);
        }

        for (_, tile) in map.iter_mut() {
            let is_land = match tile.cell_id {
                Some(id) => cell_is_land[id as usize],
                None => rng.next_bool_ratio(self.land_ratio_percent, 100),
            };

            tile.elevation = if is_land {
                rng.next_i32_inclusive(1, self.max_raise.max(1))
            } else {
                -rng.next_i32_inclusive(0, self.max_sink)
            };
            tile.terrain_kind = if is_land { TerrainKind::Land } else { TerrainKind::Water };
        }

        Ok(())
    }
}
