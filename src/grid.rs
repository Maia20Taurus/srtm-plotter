use crate::geo::*;
use crate::SrtmFrame;
use geotiff::GeoTiff;

// The minimum amount of distance (in degrees) between each pixel
// 1 degree / 3601 pixels (The dimensions of a 1 degree SRTM tile)
const RESOLUTION_DEGREES: f64 = 1.0/3601.0;

/// Create a grid of elevation points
/// min_bound and max_bound represent the bottom left and top right of the grid respectively
pub fn get_elevation_from_bounds(min_bound: &GeoPoint, max_bound: &GeoPoint) -> SrtmFrame {
    let delta_longitude = max_bound.longitude - min_bound.longitude;
    let delta_latitude = max_bound.latitude - min_bound.latitude;

    let raster_width = (delta_longitude/RESOLUTION_DEGREES) as usize;
    let raster_height = (delta_latitude/RESOLUTION_DEGREES) as usize;
    let mut elevation_grid: Vec<Vec<i16>> = vec![vec![0; raster_width]; raster_height];

    // The grid will be updated so this frame needs to be mutable
    let mut frame = SrtmFrame {
        min_bound: min_bound.clone(),
        max_bound: max_bound.clone(),
        raster_width: raster_width,
        raster_height: raster_height,
        grid: elevation_grid
    };

    frame

    // Next task: Enumerate each pixel, convert to GeoPoint and then Coord (from geo_types)
    // Then enumerate each tif file and use GeoTiff to find the file that contains the Coord
    // Then get the elevation for that point (this is not a good solution so this will be revisited later)
}

/// Use inverse linear interpolation to find the percentage of 'r1_value' between 'r1_start' and 'r1_end'
/// and then use this to linearly interpolate between 'r2_start' and 'r2_end'
fn lerp_between_ranges(r1_start: &f64, r1_end: &f64, r2_start: &f64, r2_end: &f64, r1_value: &f64) -> f64 {
    let r1_percentage = (r1_value-r1_start)/(r1_end-r1_start);
    r2_start + (r2_end-r2_start)*r1_percentage
}

/// Return a GeoPoint with the equivalent location of the provided RasterPoint
pub fn convert_raster_to_geo(frame: &SrtmFrame, point: RasterPoint) -> GeoPoint {
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

// Current task: Write unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_between_ranges() {
        assert_eq!(
            lerp_between_ranges(&5.0, &15.0, &-10.0, &0.0, &10.0),
            -5.0
        );
    }

    #[test] // Ensure that the resolution has been calculated correctly
    fn test_pixels_in_1_degree() {
        let min = GeoPoint{longitude:0.0,latitude:0.0};
        let max = GeoPoint{longitude:1.0, latitude:1.0};

        let frame = get_elevation_from_bounds(&min, &max);

        assert_eq!(frame.raster_width, 3601);
        assert_eq!(frame.raster_height, 3601)
    }
}