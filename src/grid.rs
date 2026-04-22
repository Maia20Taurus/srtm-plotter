use crate::geo::*;
use crate::SrtmFrame;
use std::fs::{File, read_dir};
use std::cmp::{min, max};
use geotiff::GeoTiff;
use geo_types::coord;

// The minimum amount of distance (in degrees) between each pixel
// 1 degree / 3601 pixels (The dimensions of a 1 degree SRTM tile)
const RESOLUTION_DEGREES: f64 = 1.0/3601.0;

const TIF_DIR: &str = "resources";

/// Enumerates all tifs in /resources/ to find one that contains 'point' and returns a GeoTiff object for that tif
/// Returns 'none' if no such tifs exist
fn get_geotiff_at_point(point: &GeoPoint) -> Option<GeoTiff> {
    for entry in read_dir(TIF_DIR).expect("Could not read TIF_DIR") {
        let path = entry.ok()?.path();
        let file = File::open(path).expect("Could not read file.");
        let geotiff = GeoTiff::read(file).expect("Could not read geotiff");

        let bounds = geotiff.model_extent();
        let min = bounds.min();
        let max = bounds.max();

        if point.longitude >= min.x && point.longitude < max.x
        && point.latitude >= min.y && point.latitude < max.y {
            return Some(geotiff);
        }
    }
    None
}

/// Returns the pixel width and height between the bounds
/// 'RasterPoint {x: raster_width, y: raster_height}'
fn compute_raster_dimensions(min_bound: &GeoPoint, max_bound: &GeoPoint) -> RasterPoint {
    let delta_longitude = max_bound.longitude - min_bound.longitude;
    let delta_latitude = max_bound.latitude - min_bound.latitude;

    let raster_width = (delta_longitude/RESOLUTION_DEGREES) as usize;
    let raster_height = (delta_latitude/RESOLUTION_DEGREES) as usize;

    RasterPoint {x: raster_width, y: raster_height}
}

/// Split the region within the bounds into subregions that each occupy one geotiff;
/// Each subregion will be at most 1 degree by 1 degree large
/// Returned frames have a zero-grid i.e. the elevation values must still be populated
fn partition_bounds(min_bound: &GeoPoint, max_bound: &GeoPoint) -> Vec<SrtmFrame> {
    let mut subframes: Vec<SrtmFrame> = Vec::new();

    for part_longitude in (min_bound.longitude.floor() as isize)..(max_bound.longitude.ceil() as isize) {
        for part_latitude in (min_bound.latitude.floor() as isize)..(max_bound.latitude.ceil() as isize) {

            let partition_min = GeoPoint {
                longitude: min_bound.longitude.max(part_longitude as f64),
                latitude: min_bound.latitude.max(part_latitude as f64)
            };
            let partition_max = GeoPoint {
                longitude: max_bound.longitude.min(part_longitude as f64),
                latitude: max_bound.latitude.min(part_latitude as f64)
            };

            let dimensions = compute_raster_dimensions(&partition_min, &partition_max);

            subframes.push(
                SrtmFrame { min_bound: partition_min, max_bound: partition_max, raster_width: dimensions.x, raster_height: dimensions.y, grid: Vec::new()}
            );
        }
    }

    subframes
}

// The 

/// Create a grid of elevation points
/// min_bound and max_bound represent the bottom left and top right of the grid respectively
pub fn get_frame_from_bounds(min_bound: &GeoPoint, max_bound: &GeoPoint) -> SrtmFrame {
    let raster_dimensions = compute_raster_dimensions(&min_bound, &max_bound);

    let raster_width = raster_dimensions.x;
    let raster_height = raster_dimensions.y;
    let elevation_grid: Vec<Vec<i16>> = vec![vec![0; raster_width]; raster_height];

    // The grid will be updated so this frame needs to be mutable
    let mut frame = SrtmFrame {
        min_bound: min_bound.clone(),
        max_bound: max_bound.clone(),
        raster_width: raster_width,
        raster_height: raster_height,
        grid: elevation_grid
    };

    for y in 0..raster_height {
        for x in 0..raster_width {
            let point = convert_raster_to_geo(&frame, &RasterPoint { x, y });
            let geotiff = get_geotiff_at_point(&point).expect("Could not find geotiff");

            frame.grid[y][x] = geotiff.get_value_at(&coord!{x:point.longitude, y:point.latitude}, 0).expect("Could not read elevation");
        }
    }

    frame

    // Next task: Enumerate each pixel, convert to GeoPoint and then Coord (from geo_types)
    // Then enumerate each tif file and use GeoTiff to find the file that contains the Coord
    // Then get the elevation for that point (this is not a good solution so this will be revisited later)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test] // Ensure that the resolution has been calculated correctly
    fn test_pixels_in_1_degree() {
        let min = GeoPoint{longitude:0.0,latitude:50.0};
        let max = GeoPoint{longitude:1.0, latitude:51.0};

        let frame = get_frame_from_bounds(&min, &max);

        assert_eq!(frame.raster_width, 3601);
        assert_eq!(frame.raster_height, 3601)
    }

    #[test]
    fn test_partition_bounds_creates_correct_number_of_subframes() {
        let min = GeoPoint{longitude:-4.3,latitude:0.0};
        let max = GeoPoint{longitude:1.0, latitude:1.2};

        assert_eq!(partition_bounds(&min, &max).len(), 12);
    }
}