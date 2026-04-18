use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HexMapError {
    InvalidDimensions,
    EmptyPipeline,
    InvalidVoronoiRegionCount,
    TectonicRequiresCellAssignments,
}

impl Display for HexMapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDimensions => write!(f, "map dimensions must be strictly positive"),
            Self::EmptyPipeline => write!(f, "pipeline has no operations"),
            Self::InvalidVoronoiRegionCount => {
                write!(f, "voronoi region count must be between 1 and map tile count")
            }
            Self::TectonicRequiresCellAssignments => {
                write!(f, "tectonic operation requires all tiles to have Voronoi cell assignments")
            }
        }
    }
}

impl Error for HexMapError {}
