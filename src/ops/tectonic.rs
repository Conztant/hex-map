use std::collections::HashMap;

use hex_grid::{CubeCoord, HexGrid};

use crate::core::error::HexMapError;
use crate::core::tile::{TerrainKind, TileData};
use crate::pipeline::operation::GeneratorOperation;
use crate::util::rng::SeededRng;

pub struct TectonicPlateOp {
    max_boundary_raise: i32,
    max_boundary_sink: i32,
    interior_jitter: i32,
}

impl TectonicPlateOp {
    pub fn new(max_boundary_raise: i32, max_boundary_sink: i32, interior_jitter: i32) -> Self {
        Self {
            max_boundary_raise: max_boundary_raise.max(0),
            max_boundary_sink: max_boundary_sink.max(0),
            interior_jitter: interior_jitter.max(0),
        }
    }
}

impl GeneratorOperation for TectonicPlateOp {
    fn name(&self) -> &'static str {
        "tectonic_plates"
    }

    fn apply(&self, map: &mut HexGrid<TileData>, rng: &mut SeededRng) -> Result<(), HexMapError> {
        let coords: Vec<CubeCoord> = map.iter().map(|(coord, _)| coord).collect();

        let coord_cell_pairs: Vec<(CubeCoord, u32)> = coords
            .iter()
            .map(|coord| {
                map.get(*coord)
                    .and_then(|tile| tile.cell_id)
                    .map(|cell_id| (*coord, cell_id))
            })
            .collect::<Option<Vec<(CubeCoord, u32)>>>()
            .ok_or(HexMapError::TectonicRequiresCellAssignments)?;

        let mut unique_cells: Vec<u32> = coord_cell_pairs.iter().map(|(_, cell_id)| *cell_id).collect();
        unique_cells.sort_unstable();
        unique_cells.dedup();

        let mut drift: HashMap<u32, bool> = HashMap::new();
        for cell_id in unique_cells {
            drift.insert(cell_id, rng.next_bool_ratio(1, 2));
        }

        for (coord, current_cell) in &coord_cell_pairs {
            let mut neighbor_cells: Vec<u32> = Vec::new();

            for (other_coord, other_cell) in &coord_cell_pairs {
                if current_cell == other_cell {
                    continue;
                }

                if map.wrapped_distance(*coord, *other_coord) == 1 {
                    neighbor_cells.push(*other_cell);
                }
            }

            let mut delta = 0;

            if !neighbor_cells.is_empty() {
                let current_drift = *drift.get(current_cell).unwrap_or(&false);
                let opposite_count = neighbor_cells
                    .iter()
                    .filter(|neighbor_cell| {
                        drift.get(neighbor_cell).copied().unwrap_or(false) != current_drift
                    })
                    .count();

                if opposite_count * 2 >= neighbor_cells.len() {
                    if self.max_boundary_raise > 0 {
                        delta += rng.next_i32_inclusive(1, self.max_boundary_raise);
                    }
                } else if self.max_boundary_sink > 0 {
                    delta -= rng.next_i32_inclusive(1, self.max_boundary_sink);
                }
            } else if self.interior_jitter > 0 {
                delta += rng.next_i32_inclusive(-self.interior_jitter, self.interior_jitter);
            }

            if let Some(tile) = map.get_mut(*coord) {
                tile.elevation += delta;
                tile.terrain_kind = if tile.elevation > 0 {
                    TerrainKind::Land
                } else {
                    TerrainKind::Water
                };
            }
        }

        Ok(())
    }
}
