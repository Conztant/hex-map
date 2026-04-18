use std::collections::{HashMap, HashSet};

use hex_grid::{CubeCoord, HexGrid, Orientation, WrappingMode};

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

        let centers = super::choose_centers(&mut coords, self.plate_count, rng);

        // Assign each tile to its nearest plate center.
        let coord_to_plate: HashMap<CubeCoord, u32> = coords
            .iter()
            .map(|&coord| {
                let id = super::nearest_center_id(coord, &centers, map) as u32;
                (coord, id)
            })
            .collect();

        // Roll drift direction once per plate (deterministic order).
        let drift: Vec<bool> = (0..self.plate_count)
            .map(|_| rng.next_bool_ratio(1, 2))
            .collect();

        // `valid_neighbors` on wrapped maps may return non-canonical (out-of-bounds) coords.
        // Normalize them to match the canonical keys stored in `coord_to_plate`.
        let (origin_q, origin_r) = coords
            .iter()
            .fold((i32::MAX, i32::MAX), |(mq, mr), c| (mq.min(c.q), mr.min(c.r)));
        let (width, height) = map.dimensions();
        let wrapping_mode = map.wrapping_mode();
        let orientation = map.orientation();
        let canonical = move |coord: CubeCoord| -> CubeCoord {
            match wrapping_mode {
                Some(WrappingMode::WrapQ) => CubeCoord::from_axial(
                    origin_q + (coord.q - origin_q).rem_euclid(width),
                    coord.r,
                ),
                Some(WrappingMode::WrapR) => CubeCoord::from_axial(
                    coord.q,
                    origin_r + (coord.r - origin_r).rem_euclid(height),
                ),
                Some(WrappingMode::Cylindrical) => match orientation {
                    Orientation::FlatTop => CubeCoord::from_axial(
                        origin_q + (coord.q - origin_q).rem_euclid(width),
                        coord.r,
                    ),
                    Orientation::PointyTop => CubeCoord::from_axial(
                        coord.q,
                        origin_r + (coord.r - origin_r).rem_euclid(height),
                    ),
                },
                Some(WrappingMode::Toroidal) => CubeCoord::from_axial(
                    origin_q + (coord.q - origin_q).rem_euclid(width),
                    origin_r + (coord.r - origin_r).rem_euclid(height),
                ),
                None => coord,
            }
        };

        for &coord in &coords {
            let current_plate = coord_to_plate[&coord];

            // BFS outward from `coord` through same-plate tiles up to `border_width` steps,
            // looking for the nearest different-plate tile.  This is O(border_width²) per tile
            // instead of the O(n) naive scan, giving O(n·border_width²) total.
            let mut frontier = vec![coord];
            let mut visited: HashSet<CubeCoord> = HashSet::from([coord]);
            let mut nearest_distance = i32::MAX;
            let mut nearest_plates: Vec<u32> = Vec::new();

            'bfs: for distance in 1..=(self.border_width as i32) {
                let mut next_frontier = Vec::new();
                for c in &frontier {
                    for neighbor_raw in map.valid_neighbors(*c) {
                        let neighbor = canonical(neighbor_raw);
                        if visited.insert(neighbor) {
                            let plate = coord_to_plate[&neighbor];
                            if plate != current_plate {
                                nearest_distance = distance;
                                if !nearest_plates.contains(&plate) {
                                    nearest_plates.push(plate);
                                }
                            } else {
                                next_frontier.push(neighbor);
                            }
                        }
                    }
                }
                if nearest_distance != i32::MAX {
                    break 'bfs;
                }
                frontier = next_frontier;
            }

            nearest_plates.sort_unstable();

            let mut delta = 0;

            if !nearest_plates.is_empty() {
                let current_drift = drift[current_plate as usize];
                let opposite_count = nearest_plates
                    .iter()
                    .filter(|&&p| drift[p as usize] != current_drift)
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

            if let Some(tile) = map.get_mut(coord) {
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
