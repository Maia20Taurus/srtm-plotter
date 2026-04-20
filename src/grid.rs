use crate::geo::*;
use crate::SrtmFrame;

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