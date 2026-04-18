use hex_grid::HexGrid;

use crate::core::error::HexMapError;
use crate::core::tile::TileData;
use crate::util::rng::SeededRng;

pub trait GeneratorOperation {
    fn name(&self) -> &'static str;
    fn apply(&self, map: &mut HexGrid<TileData>, rng: &mut SeededRng) -> Result<(), HexMapError>;
}
