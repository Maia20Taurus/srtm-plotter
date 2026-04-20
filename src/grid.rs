use crate::geo::*;
use crate::SrtmFrame;
use geotiff::GeoTiff;

// The minimum amount of distance (in degrees) between each pixel
// 30 metres (maximum resolution of SRTM) / Earth's circumference * 360 degrees
const RESOLUTION_DEGREES: f64 = 30.0 / 40075.0 * 360.0;

/// Create a grid of elevation points
/// min_bound and max_bound represent the bottom left and top right of the grid respectively
pub fn get_elevation_from_bounds(min_bound: &GeoPoint, max_bound: &GeoPoint) -> SrtmFrame {
    let delta_longitude = max_bound.longitude - max_bound.longitude;
    let delta_latitude = max_bound.latitude - min_bound.latitude;

    let raster_width = (delta_longitude/RESOLUTION_DEGREES) as usize;
    let raster_height = (delta_latitude/RESOLUTION_DEGREES) as usize;
    let mut elevation_grid: Vec<Vec<i16>> = vec![vec![0; raster_width]; raster_height];

    let mut frame = SrtmFrame {
        min_bound: min_bound.clone(),
        max_bound: max_bound.clone(),
        raster_width: raster_width,
        raster_height: raster_height,
        grid: elevation_grid
    };

    // Current task: Determine the dimensions for the grid (and then create it)

    // Next task: Enumerate each pixel, convert to GeoPoint and then Coord (from geo_types)
    // Then enumerate each tif file and use GeoTiff to find the file that contains the Coord
    // Then get the elevation for that point (this is not a good solution so this will be revisited later)
}

/// Return a GeoPoint with the equivalent location of the provided RasterPoint
pub fn convert_raster_to_geo(frame: &SrtmFrame, point: RasterPoint) -> GeoPoint {
    todo!()
}

/// Return a RasterPoint with the equivalent location of the provided GeoPoint
pub fn convert_geo_to_raster(frame: &SrtmFrame, point: GeoPoint) -> RasterPoint {
    todo!()
}