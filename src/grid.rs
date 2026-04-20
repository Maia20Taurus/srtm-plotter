use crate::geo::*;
use crate::SrtmFrame;
use geotiff::GeoTiff;

// The minimum amount of distance (in degrees) between each pixel
// 30 metres (maximum resolution of SRTM) / Earth's circumference * 360 degrees
const RESOLUTION_DEGREES: f64 = 30.0 / 40075.0 * 360.0;

/// Create a grid of elevation points
/// min_bound and max_bound represent the bottom left and top right of the grid respectively
pub fn get_elevation_from_bounds(min_bound: &GeoPoint, max_bound: &GeoPoint) -> Vec<Vec<i16>> {
    todo!()
}

/// Return a GeoPoint with the equivalent location of the provided RasterPoint
pub fn convert_raster_to_geo(frame: &SrtmFrame, point: RasterPoint) -> GeoPoint {
    todo!()
}

/// Return a RasterPoint with the equivalent location of the provided GeoPoint
pub fn convert_geo_to_raster(frame: &SrtmFrame, point: GeoPoint) -> RasterPoint {
    todo!()
}