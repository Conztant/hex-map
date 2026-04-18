use hex_grid::{HexGridConfig, Orientation, WrappingMode};

use crate::core::error::HexMapError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapShape {
    Rectangular { width: i32, height: i32 },
    OddR { columns: i32, rows: i32 },
    OddQ { columns: i32, rows: i32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapConfig {
    pub shape: MapShape,
    pub orientation: Orientation,
    pub wrapping: Option<WrappingMode>,
}

impl MapConfig {
    pub fn rectangular(
        width: i32,
        height: i32,
        orientation: Orientation,
        wrapping: Option<WrappingMode>,
    ) -> Result<Self, HexMapError> {
        if width <= 0 || height <= 0 {
            return Err(HexMapError::InvalidDimensions);
        }

        Ok(Self {
            shape: MapShape::Rectangular { width, height },
            orientation,
            wrapping,
        })
    }

    pub fn odd_r(
        columns: i32,
        rows: i32,
        orientation: Orientation,
        wrapping: Option<WrappingMode>,
    ) -> Result<Self, HexMapError> {
        if columns <= 0 || rows <= 0 {
            return Err(HexMapError::InvalidDimensions);
        }

        Ok(Self {
            shape: MapShape::OddR { columns, rows },
            orientation,
            wrapping,
        })
    }

    pub fn odd_q(
        columns: i32,
        rows: i32,
        orientation: Orientation,
        wrapping: Option<WrappingMode>,
    ) -> Result<Self, HexMapError> {
        if columns <= 0 || rows <= 0 {
            return Err(HexMapError::InvalidDimensions);
        }

        Ok(Self {
            shape: MapShape::OddQ { columns, rows },
            orientation,
            wrapping,
        })
    }

    pub fn as_hex_grid_config(self) -> HexGridConfig {
        match self.shape {
            MapShape::Rectangular { width, height } => HexGridConfig::Rectangular {
                width,
                height,
                wrapping: self.wrapping,
                orientation: self.orientation,
            },
            MapShape::OddR { columns, rows } => HexGridConfig::OddR {
                columns,
                rows,
                wrapping: self.wrapping,
                orientation: self.orientation,
            },
            MapShape::OddQ { columns, rows } => HexGridConfig::OddQ {
                columns,
                rows,
                wrapping: self.wrapping,
                orientation: self.orientation,
            },
        }
    }
}
