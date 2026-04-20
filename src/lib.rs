// Create a struct that contains elevation data for a requested rect with min and max bound
// Determine the raster grid for this rect, then query each point to get elevation

/// A pair of coordinates (in degrees) representing a point on Earth
pub struct GeoPoint {
    longitude: f64,
    latitude: f64
}

impl GeoPoint {
    /// Create a new GeoPoint
    pub fn new (longitude: f64, latitude: f64) -> Self {
        GeoPoint { longitude, latitude }
    }
}

pub struct SrtmFrame {
    // The bottom left and top right of the frame in degrees
    min_bound: GeoPoint,
    max_bound: GeoPoint,
    // The dimensions of the frame in pixels
    raster_width: u64,
    raster_height: u64,
    grid: Vec<Vec<i16>>
}

impl SrtmFrame {
    // Create a new SrtmTile
    pub fn new(min_bound: GeoPoint, max_bound: GeoPoint) -> Self {
        todo!()
    }

    pub fn get_elevation_at_pixel(latitude: u64, longitude: u64) -> i16 {
        todo!()
    }

    pub fn get_elevation_at_point(point: GeoPoint) -> i16 {
        todo!()
    }
}