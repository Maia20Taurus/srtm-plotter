/// A pair of coordinates (in degrees) representing a point on Earth
#[derive(Clone, Copy)]
pub struct GeoPoint {
    pub longitude: f64,
    pub latitude: f64
}

impl GeoPoint {
    /// Create a new GeoPoint
    pub fn new (longitude: f64, latitude: f64) -> Self {
        GeoPoint { longitude, latitude }
    }
}

/// A pair of coordinates representing a pixel on a grid, where:
/// x = longitude
/// y = latitude
#[derive(Clone, Copy)]
pub struct RasterPoint {
    pub x: usize,
    pub y: usize
}

impl RasterPoint {
    /// Create a new RasterPoint
    pub fn new(x: usize, y: usize) -> Self {
        RasterPoint { x, y }
    }
}