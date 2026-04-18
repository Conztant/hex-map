use hex_grid::{Orientation, WrappingMode};
use hex_map::{HexMapError, MapConfig, PipelineBuilder};

#[test]
fn tectonic_requires_valid_plate_count() {
    let config = MapConfig::rectangular(8, 6, Orientation::FlatTop, None).expect("valid config");

    let err = match PipelineBuilder::new()
        .tectonic_plates(0, 1, 2, 2, 1)
        .run(config, 77)
    {
        Ok(_) => panic!("tectonic with zero plate count must fail"),
        Err(err) => err,
    };

    assert_eq!(err, HexMapError::InvalidTectonicPlateCount);
}

#[test]
fn tectonic_is_deterministic_for_same_seed() {
    let config = MapConfig::rectangular(9, 7, Orientation::PointyTop, None).expect("valid config");

    let pipeline = PipelineBuilder::new()
        .land_raise_sink(58, 3, 2)
        .tectonic_plates(7, 1, 3, 2, 1)
        .build();

    let map_a = pipeline.run(config, 4242).expect("first run should succeed");
    let map_b = pipeline
        .run(
            MapConfig::rectangular(9, 7, Orientation::PointyTop, None).expect("valid config"),
            4242,
        )
        .expect("second run should succeed");

    for (coord, tile_a) in map_a.iter() {
        let tile_b = map_b.get(coord).expect("matching tile in second map");
        assert_eq!(tile_a.cell_id, tile_b.cell_id);
        assert_eq!(tile_a.elevation, tile_b.elevation);
        assert_eq!(tile_a.terrain_kind, tile_b.terrain_kind);
    }
}

#[test]
fn tectonic_changes_elevation_distribution() {
    let config = MapConfig::rectangular(10, 8, Orientation::FlatTop, None).expect("valid config");

    let base_map = PipelineBuilder::new()
        .voronoi(8)
        .land_raise_sink(55, 3, 3)
        .run(config, 2026)
        .expect("base pipeline should run");

    let tectonic_map = PipelineBuilder::new()
        .land_raise_sink(55, 3, 3)
        .tectonic_plates(8, 1, 3, 3, 1)
        .run(
            MapConfig::rectangular(10, 8, Orientation::FlatTop, None).expect("valid config"),
            2026,
        )
        .expect("tectonic pipeline should run");

    let changed_tiles = base_map
        .iter()
        .filter(|(coord, base_tile)| {
            tectonic_map
                .get(*coord)
                .map(|tile| tile.elevation != base_tile.elevation)
                .unwrap_or(false)
        })
        .count();

    assert!(changed_tiles > 0, "tectonic step should alter some elevations");
}

#[test]
fn tectonic_runs_with_wrapping_enabled() {
    let config = MapConfig::rectangular(
        10,
        8,
        Orientation::FlatTop,
        Some(WrappingMode::Toroidal),
    )
    .expect("valid config");

    let map = PipelineBuilder::new()
        .land_raise_sink(55, 3, 3)
        .tectonic_plates(8, 1, 3, 3, 1)
        .run(config, 2026)
        .expect("tectonic pipeline should run with wrapping");

    assert!(map.len() > 0);
}

#[test]
fn wider_border_affects_more_tiles() {
    let config = MapConfig::rectangular(16, 12, Orientation::FlatTop, None).expect("valid config");

    let base_map = PipelineBuilder::new()
        .land_raise_sink(55, 3, 3)
        .run(config, 88)
        .expect("base pipeline should run");

    let narrow_border = PipelineBuilder::new()
        .land_raise_sink(55, 3, 3)
        .tectonic_plates(6, 1, 3, 3, 0)
        .run(
            MapConfig::rectangular(16, 12, Orientation::FlatTop, None).expect("valid config"),
            88,
        )
        .expect("narrow border pipeline should run");

    let wide_border = PipelineBuilder::new()
        .land_raise_sink(55, 3, 3)
        .tectonic_plates(6, 3, 3, 3, 0)
        .run(
            MapConfig::rectangular(16, 12, Orientation::FlatTop, None).expect("valid config"),
            88,
        )
        .expect("wide border pipeline should run");

    let changed_narrow = base_map
        .iter()
        .filter(|(coord, base_tile)| {
            narrow_border
                .get(*coord)
                .map(|tile| tile.elevation != base_tile.elevation)
                .unwrap_or(false)
        })
        .count();

    let changed_wide = base_map
        .iter()
        .filter(|(coord, base_tile)| {
            wide_border
                .get(*coord)
                .map(|tile| tile.elevation != base_tile.elevation)
                .unwrap_or(false)
        })
        .count();

    assert!(
        changed_wide >= changed_narrow,
        "wider border should affect at least as many tiles"
    );
}
