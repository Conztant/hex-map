use hex_grid::Orientation;
use hex_map::{MapConfig, PipelineBuilder};

#[test]
fn same_seed_same_pipeline_produces_same_map() {
    let config = MapConfig::rectangular(9, 7, Orientation::FlatTop, None).expect("valid config");

    let pipeline = PipelineBuilder::new().voronoi(6).land_raise_sink(55, 3, 2).build();

    let map_a = pipeline.run(config, 12345).expect("first run should succeed");
    let map_b = pipeline.run(config, 12345).expect("second run should succeed");

    assert_eq!(map_a.len(), map_b.len());

    for (coord, tile_a) in map_a.iter() {
        let tile_b = map_b.get(coord).expect("matching tile in second map");
        assert_eq!(tile_a.cell_id, tile_b.cell_id);
        assert_eq!(tile_a.elevation, tile_b.elevation);
        assert_eq!(tile_a.terrain_kind, tile_b.terrain_kind);
    }
}
