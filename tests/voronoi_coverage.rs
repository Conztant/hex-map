use hex_grid::{Orientation, WrappingMode};
use hex_map::{MapConfig, PipelineBuilder};

#[test]
fn voronoi_assigns_every_active_tile() {
    let config = MapConfig::odd_r(
        10,
        8,
        Orientation::PointyTop,
        Some(WrappingMode::Toroidal),
    )
    .expect("valid config");

    let map = PipelineBuilder::new()
        .voronoi(9)
        .run(config, 99)
        .expect("voronoi generation should succeed");

    for (_, tile) in map.iter() {
        assert!(tile.cell_id.is_some());
    }
}
