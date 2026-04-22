use crate::SrtmFrame;
use crate::grid::get_elevation_in_bounds;

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

/// Use inverse linear interpolation to find the percentage of 'r1_value' between 'r1_start' and 'r1_end'
/// and then use this to linearly interpolate between 'r2_start' and 'r2_end'
fn lerp_between_ranges(r1_start: &f64, r1_end: &f64, r2_start: &f64, r2_end: &f64, r1_value: &f64) -> f64 {
    let r1_percentage = (r1_value-r1_start)/(r1_end-r1_start);
    r2_start + (r2_end-r2_start)*r1_percentage
}

/// Return a GeoPoint with the equivalent location of the provided RasterPoint
pub fn convert_raster_to_geo(frame: &SrtmFrame, point: &RasterPoint) -> GeoPoint {
    GeoPoint {
        longitude: lerp_between_ranges(
            &0.0,
            &(frame.raster_width as f64),
            &frame.min_bound.longitude,
            &frame.max_bound.longitude,
            &(point.x as f64)),
        latitude: lerp_between_ranges(
            &0.0,
            &(frame.raster_height as f64),
            &frame.min_bound.latitude,
            &frame.max_bound.latitude,
            &(point.y as f64))
    }
}


/// Return a RasterPoint with the equivalent location of the provided GeoPoint
pub fn convert_geo_to_raster(frame: &SrtmFrame, point: &GeoPoint) -> RasterPoint {
    RasterPoint {
        x: lerp_between_ranges(
            &frame.min_bound.longitude,
            &frame.max_bound.longitude,
            &0.0,
            &(frame.raster_width as f64),
            &point.longitude) as usize,
        y: lerp_between_ranges(
            &frame.min_bound.latitude,
            &frame.max_bound.latitude,
            &0.0,
            &(frame.raster_height as f64),
            &point.latitude) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use almost;

   #[test]
    fn test_lerp_between_ranges() {
        assert_eq!(
            lerp_between_ranges(&5.0, &15.0, &-10.0, &0.0, &10.0),
            -5.0
        );
    }

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

        assert_eq!(raster.x,(0.5*3601.0) as usize);
        assert_eq!(raster.y,(0.8*3601.0) as usize);
    }
}