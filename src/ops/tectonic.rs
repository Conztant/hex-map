use std::collections::HashMap;

use hex_grid::{CubeCoord, HexGrid};

use crate::core::error::HexMapError;
use crate::core::tile::{TerrainKind, TileData};
use crate::pipeline::operation::GeneratorOperation;
use crate::util::rng::SeededRng;

pub struct TectonicPlateOp {
    plate_count: usize,
    border_width: usize,
    max_boundary_raise: i32,
    max_boundary_sink: i32,
    interior_jitter: i32,
}

impl TectonicPlateOp {
    pub fn new(
        plate_count: usize,
        border_width: usize,
        max_boundary_raise: i32,
        max_boundary_sink: i32,
        interior_jitter: i32,
    ) -> Self {
        Self {
            plate_count,
            border_width: border_width.max(1),
            max_boundary_raise: max_boundary_raise.max(0),
            max_boundary_sink: max_boundary_sink.max(0),
            interior_jitter: interior_jitter.max(0),
        }
    }

    fn choose_centers(&self, coords: &mut [CubeCoord], rng: &mut SeededRng) -> Vec<CubeCoord> {
        rng.shuffle(coords);
        coords[..self.plate_count].to_vec()
    }

    fn scale_boundary_magnitude(&self, magnitude: i32, boundary_distance: i32) -> i32 {
        if magnitude <= 0 {
            return 0;
        }

        if boundary_distance <= 0 || boundary_distance as usize > self.border_width {
            return 0;
        }

        let weight = (self.border_width - boundary_distance as usize + 1) as i32;
        let scaled = magnitude * weight / self.border_width as i32;
        scaled.max(1)
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
            let mut nearest_plates: Vec<u32> = Vec::new();
            let mut nearest_distance = i32::MAX;

            for (other_coord, other_plate) in &coord_plate_pairs {
                if current_plate == other_plate {
                    continue;
                }

                let distance = map.wrapped_distance(*coord, *other_coord);

                if distance < nearest_distance {
                    nearest_distance = distance;
                    nearest_plates.clear();
                    nearest_plates.push(*other_plate);
                } else if distance == nearest_distance {
                    nearest_plates.push(*other_plate);
                }
            }

            nearest_plates.sort_unstable();
            nearest_plates.dedup();

            let mut delta = 0;

            if !nearest_plates.is_empty() && nearest_distance as usize <= self.border_width {
                let current_drift = *drift.get(current_plate).unwrap_or(&false);
                let opposite_count = nearest_plates
                    .iter()
                    .filter(|neighbor_plate| {
                        drift.get(neighbor_plate).copied().unwrap_or(false) != current_drift
                    })
                    .count();

                if opposite_count * 2 >= nearest_plates.len() {
                    if self.max_boundary_raise > 0 {
                        let base = rng.next_i32_inclusive(1, self.max_boundary_raise);
                        delta += self.scale_boundary_magnitude(base, nearest_distance);
                    }
                } else if self.max_boundary_sink > 0 {
                    let base = rng.next_i32_inclusive(1, self.max_boundary_sink);
                    delta -= self.scale_boundary_magnitude(base, nearest_distance);
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
