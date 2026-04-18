use hex_grid::{Orientation, WrappingMode};
use hex_map::{MapConfig, PipelineBuilder};

#[test]
fn rectangular_supports_all_wrapping_modes() {
    let cases = [
        None,
        Some(WrappingMode::WrapQ),
        Some(WrappingMode::WrapR),
        Some(WrappingMode::Cylindrical),
        Some(WrappingMode::Toroidal),
    ];

    for wrapping in cases {
        let config = MapConfig::rectangular(8, 6, Orientation::FlatTop, wrapping).expect("valid config");
        let map = PipelineBuilder::new()
            .voronoi(5)
            .land_raise_sink(50, 2, 2)
            .run(config, 42)
            .expect("pipeline should run");

        for (_, tile) in map.iter() {
            assert!(tile.cell_id.is_some());
        }
    }
}

#[test]
fn odd_shapes_run_with_wrapping_and_without_wrapping() {
    let odd_r_none = MapConfig::odd_r(8, 6, Orientation::PointyTop, None).expect("valid config");
    let odd_r_wrap = MapConfig::odd_r(
        8,
        6,
        Orientation::PointyTop,
        Some(WrappingMode::Cylindrical),
    )
    .expect("valid config");
    let odd_q_none = MapConfig::odd_q(8, 6, Orientation::FlatTop, None).expect("valid config");
    let odd_q_wrap =
        MapConfig::odd_q(8, 6, Orientation::FlatTop, Some(WrappingMode::Toroidal))
            .expect("valid config");

    let configs = [odd_r_none, odd_r_wrap, odd_q_none, odd_q_wrap];

    for config in configs {
        let map = PipelineBuilder::new()
            .voronoi(4)
            .land_raise_sink(65, 3, 2)
            .run(config, 1337)
            .expect("pipeline should run");

        assert!(map.len() > 0);
    }
}
