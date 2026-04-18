# hex-map

Data-only modular hex map generation library built on top of `hex-grid`.

## Goals

- Pure data generation (no graphics concerns)
- CyberChef-like linear operation chaining
- Deterministic seeded generation
- Shape and wrapping support delegated to `hex-grid`
- MVP generators: Voronoi partition + land raise/sink

## Dependency Policy

This crate uses only:

- `hex-grid`
- Rust standard library

## Quick Start

```rust
use hex_grid::{Orientation, WrappingMode};
use hex_map::{MapConfig, PipelineBuilder};

let config = MapConfig::rectangular(
    24,
    16,
    Orientation::FlatTop,
    Some(WrappingMode::Toroidal),
)?;

let map = PipelineBuilder::new()
    .voronoi(12)
    .land_raise_sink(60, 4, 3)
    .tectonic_plates(12, 2, 2, 1)
    .run(config, 2026)?;

assert_eq!(map.len(), 24 * 16);
```

## Current Operations

- `VoronoiPartitionOp`: partitions all active tiles into Voronoi regions (every tile receives a region id)
- `LandRaiseSinkOp`: marks tiles as land/water and assigns elevation
- `TectonicPlateOp`: applies deterministic boundary uplift/subduction and interior jitter by Voronoi plate

## Notes

- Wrapping-aware distance is used through `hex-grid` API (`wrapped_distance`)
- Tie-breaking for equal Voronoi distance is deterministic (lowest center index)
