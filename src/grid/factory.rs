use hex_grid::HexGrid;

use crate::core::config::MapConfig;
use crate::core::tile::TileData;

pub fn build_grid(config: MapConfig) -> HexGrid<TileData> {
    HexGrid::from_config(config.as_hex_grid_config(), |_| TileData::default())
}
