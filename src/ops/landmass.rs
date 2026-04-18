use std::collections::HashMap;

use hex_grid::{CubeCoord, HexGrid};

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
        // Collect all unique cell IDs present in the map (sorted for determinism).
        let mut unique_ids: Vec<u32> = {
            let mut ids: Vec<u32> = map
                .iter()
                .filter_map(|(_, tile)| tile.cell_id)
                .collect();
            ids.sort_unstable();
            ids.dedup();
            ids
        };
        // sort_unstable already produced a sorted deduplicated list, but we call
        // dedup after sort so the order is stable across runs with the same seed.
        unique_ids.sort_unstable();

        // Roll land/water once per Voronoi cell so every tile in a cell gets the
        // same terrain kind → produces large, continuous regions instead of noise.
        let cell_is_land: HashMap<u32, bool> = unique_ids
            .iter()
            .map(|&id| (id, rng.next_bool_ratio(self.land_ratio_percent, 100)))
            .collect();

        let coords: Vec<CubeCoord> = map.iter().map(|(coord, _)| coord).collect();

        for coord in coords {
            // Determine land/water: use the cell decision when available, else
            // fall back to a per-tile roll for tiles not covered by Voronoi.
            let is_land = match map.get(coord).and_then(|t| t.cell_id) {
                Some(id) => *cell_is_land.get(&id).unwrap_or(&false),
                None => rng.next_bool_ratio(self.land_ratio_percent, 100),
            };

            let elevation = if is_land {
                rng.next_i32_inclusive(1, self.max_raise.max(1))
            } else {
                -rng.next_i32_inclusive(0, self.max_sink)
            };

            if let Some(tile) = map.get_mut(coord) {
                tile.elevation = elevation;
                tile.terrain_kind = if is_land {
                    TerrainKind::Land
                } else {
                    TerrainKind::Water
                };
            }
        }

        Ok(())
    }
}
