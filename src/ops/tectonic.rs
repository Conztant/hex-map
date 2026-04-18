use std::collections::HashMap;

use hex_grid::{CubeCoord, HexGrid};

use crate::core::error::HexMapError;
use crate::core::tile::{TerrainKind, TileData};
use crate::pipeline::operation::GeneratorOperation;
use crate::util::rng::SeededRng;

pub struct TectonicPlateOp {
    plate_count: usize,
    max_boundary_raise: i32,
    max_boundary_sink: i32,
    interior_jitter: i32,
}

impl TectonicPlateOp {
    pub fn new(
        plate_count: usize,
        max_boundary_raise: i32,
        max_boundary_sink: i32,
        interior_jitter: i32,
    ) -> Self {
        Self {
            plate_count,
            max_boundary_raise: max_boundary_raise.max(0),
            max_boundary_sink: max_boundary_sink.max(0),
            interior_jitter: interior_jitter.max(0),
        }
    }

    fn choose_centers(&self, coords: &mut [CubeCoord], rng: &mut SeededRng) -> Vec<CubeCoord> {
        rng.shuffle(coords);
        coords[..self.plate_count].to_vec()
    }
}

impl GeneratorOperation for TectonicPlateOp {
    fn name(&self) -> &'static str {
        "tectonic_plates"
    }

    fn apply(&self, map: &mut HexGrid<TileData>, rng: &mut SeededRng) -> Result<(), HexMapError> {
        let mut coords: Vec<CubeCoord> = map.iter().map(|(coord, _)| coord).collect();

        if self.plate_count == 0 || self.plate_count > coords.len() {
            return Err(HexMapError::InvalidTectonicPlateCount);
        }

        let centers = self.choose_centers(&mut coords, rng);
        let all_coords: Vec<CubeCoord> = map.iter().map(|(coord, _)| coord).collect();
        let coord_plate_pairs: Vec<(CubeCoord, u32)> = all_coords
            .iter()
            .map(|coord| {
                let mut best_id = 0usize;
                let mut best_distance = i32::MAX;

                for (idx, center) in centers.iter().enumerate() {
                    let distance = map.wrapped_distance(*coord, *center);
                    if distance < best_distance || (distance == best_distance && idx < best_id) {
                        best_distance = distance;
                        best_id = idx;
                    }
                }

                (*coord, best_id as u32)
            })
            .collect();

        let mut drift: HashMap<u32, bool> = HashMap::new();
        for plate_id in 0..self.plate_count {
            drift.insert(plate_id as u32, rng.next_bool_ratio(1, 2));
        }

        for (coord, current_plate) in &coord_plate_pairs {
            let mut neighbor_plates: Vec<u32> = Vec::new();

            for (other_coord, other_plate) in &coord_plate_pairs {
                if current_plate == other_plate {
                    continue;
                }

                if map.wrapped_distance(*coord, *other_coord) == 1 {
                    neighbor_plates.push(*other_plate);
                }
            }

            let mut delta = 0;

            if !neighbor_plates.is_empty() {
                let current_drift = *drift.get(current_plate).unwrap_or(&false);
                let opposite_count = neighbor_plates
                    .iter()
                    .filter(|neighbor_plate| {
                        drift.get(neighbor_plate).copied().unwrap_or(false) != current_drift
                    })
                    .count();

                if opposite_count * 2 >= neighbor_plates.len() {
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
