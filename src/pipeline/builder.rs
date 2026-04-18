use crate::core::config::MapConfig;
use crate::core::error::HexMapError;
use crate::core::tile::TileData;
use crate::ops::landmass::LandRaiseSinkOp;
use crate::ops::tectonic::TectonicPlateOp;
use crate::ops::voronoi::VoronoiPartitionOp;
use crate::pipeline::pipeline::Pipeline;

pub struct PipelineBuilder {
    pipeline: Pipeline,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            pipeline: Pipeline::new(),
        }
    }

    pub fn voronoi(mut self, region_count: usize) -> Self {
        self.pipeline.add_operation(VoronoiPartitionOp::new(region_count));
        self
    }

    pub fn land_raise_sink(
        mut self,
        land_ratio_percent: u32,
        max_raise: i32,
        max_sink: i32,
    ) -> Self {
        self.pipeline.add_operation(LandRaiseSinkOp::new(
            land_ratio_percent,
            max_raise,
            max_sink,
        ));
        self
    }

    pub fn tectonic_plates(
        mut self,
        max_boundary_raise: i32,
        max_boundary_sink: i32,
        interior_jitter: i32,
    ) -> Self {
        self.pipeline.add_operation(TectonicPlateOp::new(
            max_boundary_raise,
            max_boundary_sink,
            interior_jitter,
        ));
        self
    }

    pub fn build(self) -> Pipeline {
        self.pipeline
    }

    pub fn run(self, config: MapConfig, seed: u64) -> Result<hex_grid::HexGrid<TileData>, HexMapError> {
        self.pipeline.run(config, seed)
    }
}

impl Default for PipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
