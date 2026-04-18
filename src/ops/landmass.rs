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
        let coords: Vec<CubeCoord> = map.iter().map(|(coord, _)| coord).collect();

        for coord in coords {
            let is_land = rng.next_bool_ratio(self.land_ratio_percent, 100);
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
