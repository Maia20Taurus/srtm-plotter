use crate::geo::*;
use crate::SrtmFrame;
use std::fs::{File, read_dir};
use std::vec;
use geotiff::GeoTiff;
use geo_types::coord;

// The minimum amount of distance (in degrees) between each pixel
// 1 degree / 3601 pixels (The dimensions of a 1 degree SRTM tile)
pub const RESOLUTION_DEGREES: f64 = 1.0/3601.0;

const TIF_DIR: &str = "resources";

/// Returns the pixel width and height between the bounds
/// 'RasterPoint {x: raster_width, y: raster_height}'
fn compute_raster_dimensions(min_bound: &GeoPoint, max_bound: &GeoPoint) -> RasterPoint {
    let delta_longitude = max_bound.longitude - min_bound.longitude;
    let delta_latitude = max_bound.latitude - min_bound.latitude;

    let raster_width = (delta_longitude/RESOLUTION_DEGREES).round() as usize;
    let raster_height = (delta_latitude/RESOLUTION_DEGREES).round() as usize;

    RasterPoint {x: raster_width, y: raster_height}
}

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

/// Populate the frame's elevation grid using GeoTiff data
/// This function expects the frame to fit inside a single GeoTiff (see partition_bounds to achieve this)
/// () if no GeoTiff is found for any of the pixels in the frame
fn fill_frame_elevation_grid(mut frame: SrtmFrame) -> Option<SrtmFrame> {
    let mid = GeoPoint{
        longitude:(frame.max_bound.longitude+frame.min_bound.longitude)/2.0,
        latitude:(frame.max_bound.latitude+frame.min_bound.latitude)/2.0
    };

    let geotiff = get_geotiff_at_point(&mid)?;

    for y in 0..frame.raster_height {
        for x in 0..frame.raster_width {
            let pixel = RasterPoint{x, y};
            let coordinate = convert_raster_to_geo(&frame, &pixel);

            frame.grid[y][x] = geotiff.get_value_at(
                &coord!{
                    // Clamp the coordinates inside the geotiff to prevent queries at its edges
                    x:coordinate.longitude,
                    y:coordinate.latitude},
                    0
            ).unwrap_or(0);
        }
    }

    Some(frame)
}

fn fill_frame_with_partition(main_frame: &mut SrtmFrame, partition_min: GeoPoint, partition_max: GeoPoint) {
    let geotiff = get_geotiff_at_point(&partition_min).expect("geotiff does not exist");

    let partition_dimensions = compute_raster_dimensions(&partition_min, &partition_max);
    let partition_start_pixel = convert_geo_to_raster(&main_frame, &partition_min);

    // Initialise a partition frame to allow use of the coordinates conversion functions
    let partition_frame = SrtmFrame {
        min_bound: partition_min.clone(),
        max_bound: partition_max.clone(),
        raster_width: partition_dimensions.x,
        raster_height: partition_dimensions.y,
        grid: vec![vec![0; 0]; 0] // Empty vec needed just to initialise the frame
    };

    for part_y in 0..partition_dimensions.y {
        for part_x in 0..partition_dimensions.x {

            let current_pixel_main = RasterPoint{
                x: partition_start_pixel.x + part_x,
                y: partition_start_pixel.y + part_y 
            };
            let current_pixel_partition = RasterPoint{
                x: part_x,
                y: part_y
            };

            let current_point = convert_raster_to_geo(&partition_frame, &current_pixel_partition);

            main_frame.grid[current_pixel_main.y][current_pixel_main.x] = geotiff.get_value_at(
                &coord!{x:current_point.longitude,y:current_point.latitude},
                0).unwrap_or(0);
        }
    }
}

pub fn get_elevation_in_bounds(min_bound: &GeoPoint, max_bound: &GeoPoint) -> SrtmFrame {
    let main_dimensions = compute_raster_dimensions(&min_bound, &max_bound);

    let mut main_frame = SrtmFrame {
        min_bound:min_bound.clone(),
        max_bound:max_bound.clone(),
        raster_width:main_dimensions.x,
        raster_height:main_dimensions.y,
        grid: vec![vec![0; main_dimensions.x]; main_dimensions.y]
    };

    for p_lat in (min_bound.latitude.floor() as isize)..(max_bound.latitude.ceil() as isize) {
        for p_lon in (min_bound.longitude.floor() as isize)..(max_bound.longitude.ceil() as isize) {

            let partition_min = GeoPoint {
                longitude:min_bound.longitude.max(p_lon as f64),
                latitude:min_bound.latitude.max(p_lat as f64)
            };
            let partition_max = GeoPoint {
                longitude:max_bound.longitude.min((p_lon+1) as f64),
                latitude:max_bound.latitude.min((p_lat+1) as f64)
            };

            fill_frame_with_partition(
                &mut main_frame,
                partition_min,
                partition_max
            );
        }
    }

    main_frame
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test] // Ensure that the resolution has been calculated correctly
    fn test_pixels_in_1_degree() {
        let min = GeoPoint{longitude:0.0,latitude:50.0};
        let max = GeoPoint{longitude:1.0, latitude:51.0};

        let frame = get_elevation_in_bounds(&min, &max);

        assert_eq!(frame.raster_width, 3601);
        assert_eq!(frame.raster_height, 3601)
    }

    #[test]
    fn test_pixels_in_half_degree() {
        let min = GeoPoint{longitude:0.0,latitude:50.0};
        let max = GeoPoint{longitude:0.5, latitude:50.5};

        let frame = get_elevation_in_bounds(&min, &max);

        assert_eq!(frame.raster_width, 1800);
        assert_eq!(frame.raster_height, 1800)
    }

    #[test]
    fn test_partition_bounds_creates_multiple_subframes() {
        let min = GeoPoint{longitude:-4.3,latitude:50.0};
        let max = GeoPoint{longitude:1.0, latitude:51.2};

        assert_eq!(get_elevation_in_bounds(&min, &max).grid.len(), 12);
    }

    #[test]
    fn test_partition_bounds_creates_one_subframe() {
        let min = GeoPoint{longitude:-3.8,latitude:50.0};
        let max = GeoPoint{longitude:-3.2, latitude:50.6};

        assert_eq!(get_elevation_in_bounds(&min, &max).grid.len(), 1);
    }
}