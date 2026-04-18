use hex_grid::HexGrid;

use crate::core::config::MapConfig;
use crate::core::error::HexMapError;
use crate::core::tile::TileData;
use crate::grid::factory::build_grid;
use crate::pipeline::operation::GeneratorOperation;
use crate::util::rng::SeededRng;

pub struct Pipeline {
    operations: Vec<Box<dyn GeneratorOperation>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub fn add_operation<O>(&mut self, op: O)
    where
        O: GeneratorOperation + 'static,
    {
        self.operations.push(Box::new(op));
    }

    pub fn run(&self, config: MapConfig, seed: u64) -> Result<HexGrid<TileData>, HexMapError> {
        if self.operations.is_empty() {
            return Err(HexMapError::EmptyPipeline);
        }

        let mut map = build_grid(config);
        let mut rng = SeededRng::new(seed);

        for op in &self.operations {
            op.apply(&mut map, &mut rng)?;
        }

        Ok(map)
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
