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