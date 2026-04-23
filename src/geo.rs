use crate::SrtmFrame;
use crate::grid::RESOLUTION_DEGREES;

/// A pair of coordinates (in degrees) representing a point on Earth
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
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

/// Return a GeoPoint with the equivalent location of the provided RasterPoint
pub fn convert_raster_to_geo(frame: &SrtmFrame, point: &RasterPoint) -> GeoPoint {
    GeoPoint {
        longitude: frame.min_bound.longitude + (point.x as f64) * RESOLUTION_DEGREES,
        latitude: frame.max_bound.latitude - (point.y as f64) * RESOLUTION_DEGREES
    }
}


/// Return a RasterPoint with the equivalent location of the provided GeoPoint
pub fn convert_geo_to_raster(frame: &SrtmFrame, point: &GeoPoint) -> RasterPoint {
    RasterPoint {
        x: ((point.longitude - frame.min_bound.longitude) / RESOLUTION_DEGREES).round() as usize,
        y: ((point.latitude - frame.min_bound.latitude) / RESOLUTION_DEGREES).round() as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::get_elevation_in_bounds;
    use super::*;
    use almost;

    #[test]
    fn test_convert_raster_to_geo() {
        let min = GeoPoint{longitude:0.0,latitude:50.0};
        let max = GeoPoint{longitude:1.0, latitude:51.0};
        let frame = get_elevation_in_bounds(&min, &max);

        let point = RasterPoint{x:1800,y:1500};
        let geo = convert_raster_to_geo(&frame, &point);

        assert!(almost::equal(geo.longitude, 1800.0/3601.0));
        assert!(almost::equal(geo.latitude, 50.0+1500.0/3601.0));
        
    }

    #[test]
    fn test_convert_geo_to_raster() {
        let min = GeoPoint{longitude:0.0,latitude:50.0};
        let max = GeoPoint{longitude:1.0, latitude:51.0};
        let frame = get_elevation_in_bounds(&min, &max);

        let point = GeoPoint{longitude:0.5,latitude:50.8};
        let raster = convert_geo_to_raster(&frame, &point);

        assert_eq!(raster.x,1801 as usize);
        assert_eq!(raster.y,2881 as usize);
    }
}