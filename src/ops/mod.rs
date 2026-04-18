pub mod landmass;
pub mod tectonic;
pub mod voronoi;

use hex_grid::{CubeCoord, HexGrid};

use crate::core::tile::TileData;
use crate::util::rng::SeededRng;

/// Shuffles `coords` in place and returns the first `count` elements as the chosen centers.
pub(crate) fn choose_centers(
    coords: &mut [CubeCoord],
    count: usize,
    rng: &mut SeededRng,
) -> Vec<CubeCoord> {
    rng.shuffle(coords);
    coords[..count].to_vec()
}

/// Returns the index of the center in `centers` closest to `coord` (wrapped distance,
/// ties broken by lower index).
pub(crate) fn nearest_center_id(
    coord: CubeCoord,
    centers: &[CubeCoord],
    map: &HexGrid<TileData>,
) -> usize {
    let mut best_id = 0usize;
    let mut best_distance = i32::MAX;
    for (idx, &center) in centers.iter().enumerate() {
        let distance = map.wrapped_distance(coord, center);
        if distance < best_distance || (distance == best_distance && idx < best_id) {
            best_distance = distance;
            best_id = idx;
        }
    }
    best_id
}
