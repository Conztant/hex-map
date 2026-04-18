pub mod core;
pub mod grid;
pub mod ops;
pub mod pipeline;
pub mod util;

pub use core::config::{MapConfig, MapShape};
pub use core::error::HexMapError;
pub use core::tile::{TerrainKind, TileData};
pub use ops::landmass::LandRaiseSinkOp;
pub use ops::voronoi::VoronoiPartitionOp;
pub use pipeline::builder::PipelineBuilder;
pub use pipeline::pipeline::Pipeline;

#[cfg(test)]
mod tests {
    use hex_grid::Orientation;

    use crate::core::config::MapConfig;
    use crate::pipeline::builder::PipelineBuilder;

    #[test]
    fn pipeline_runs_smoke() {
        let config = MapConfig::rectangular(5, 5, Orientation::FlatTop, None)
            .expect("valid map config");

        let map = PipelineBuilder::new()
            .voronoi(4)
            .land_raise_sink(50, 2, 2)
            .run(config, 7)
            .expect("pipeline should run");

        assert_eq!(map.len(), 25);
    }
}
