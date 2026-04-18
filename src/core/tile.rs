#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainKind {
    Water,
    Land,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileData {
    pub cell_id: Option<u32>,
    pub elevation: i32,
    pub terrain_kind: TerrainKind,
}

impl Default for TileData {
    fn default() -> Self {
        Self {
            cell_id: None,
            elevation: 0,
            terrain_kind: TerrainKind::Water,
        }
    }
}
