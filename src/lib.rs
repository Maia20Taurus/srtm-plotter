// Create a struct that contains elevation data for a requested rect with min and max bound
// Determine the raster grid for this rect, then query each point to get elevation

pub mod geo;
pub mod grid;

use geo::*;
use grid::*;

/// Contains elevation data for a specified boundary from the SRTM dataset
pub struct SrtmFrame {
    // The bottom left and top right of the frame in degrees
    pub min_bound: GeoPoint,
    pub max_bound: GeoPoint,
    // The dimensions of the frame in pixels
    pub raster_width: usize,
    pub raster_height: usize,
    pub grid: Vec<Vec<i16>>
}

impl SrtmFrame {
    // Create a new SrtmTile
    pub fn new(min_bound: &GeoPoint, max_bound: &GeoPoint) -> Self {
        get_frame_from_bounds(&min_bound, &max_bound)
    }

    /// Get the elevation at the specified pixel
    pub fn get_elevation_at_pixel(&self, latitude: usize, longitude: usize) -> i16 {
        self.grid[latitude][longitude]
    }

    /// Get the elevation at the specified GeoPoint
    pub fn get_elevation_at_point(&self, point: GeoPoint) -> i16 {
        todo!()
    }
}