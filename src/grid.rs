use crate::geo::*;
use crate::SrtmFrame;
use std::fs::{File, read_dir};
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

    let raster_width = (delta_longitude/RESOLUTION_DEGREES) as usize;
    let raster_height = (delta_latitude/RESOLUTION_DEGREES) as usize;

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

        if point.longitude > min.x && point.longitude < max.x
        && point.latitude > min.y && point.latitude < max.y {
            return Some(geotiff);
        }
    }
    None
}

/// Populate the frame's elevation grid using GeoTiff data
/// This function expects the frame to fit inside a single GeoTiff (see partition_bounds to achieve this)
/// () if no GeoTiff is found for any of the pixels in the frame
fn fill_frame_elevation_grid(mut frame: SrtmFrame) -> Option<SrtmFrame> {
    let geotiff = get_geotiff_at_point(&frame.min_bound).expect("GeoTiff does not exist");

    let extent = geotiff.model_extent();
    let tif_min = extent.min();
    let tif_max = extent.max();

    let eps = 1e-12;

    for y in 0..frame.raster_height {
        for x in 0..frame.raster_width {
            let pixel = RasterPoint{x, y};
            let coordinate = convert_raster_to_geo(&frame, &pixel);

            frame.grid[y][x] = geotiff.get_value_at(
                &coord!{
                    // Clamp the coordinates inside the geotiff to prevent queries at its edges
                    x:coordinate.longitude.clamp(tif_min.x+eps, tif_max.x-eps),
                    y:coordinate.latitude.clamp(tif_min.y+eps, tif_max.y-eps)},
                    0
            ).expect("Could not get elevation");
        }
    }

    Some(frame)
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
                longitude: max_bound.longitude.min(part_longitude as f64 + 1.0),
                latitude: max_bound.latitude.min(part_latitude as f64 + 1.0)
            };

            let dimensions = compute_raster_dimensions(&partition_min, &partition_max);

            let subframe = SrtmFrame {
                min_bound: partition_min, 
                max_bound: partition_max, 
                raster_width: dimensions.x, 
                raster_height: dimensions.y, 
                grid: vec![vec![0; dimensions.x]; dimensions.y]};

            let populated_subframe = fill_frame_elevation_grid(subframe).expect("GeoTiff does not exist for this subframe");

            subframes.push(
                populated_subframe
            );
        }
    }

    subframes
}

/// Return an SrtmFrame containing elevation data within 'min_bound' and 'max_bound'
/// Returns 'None' if there is missing data for any coordinates inside the frame
pub fn get_elevation_in_bounds(min_bound: &GeoPoint, max_bound: &GeoPoint) -> SrtmFrame {
    let dimensions = compute_raster_dimensions(&min_bound, &max_bound);
    let mut main_frame = SrtmFrame{
        min_bound:min_bound.clone(),
        max_bound:max_bound.clone(),
        raster_width: dimensions.x,
        raster_height: dimensions.y,
        grid: vec![vec![0; dimensions.x]; dimensions.y]
    };

    let sub_frames = partition_bounds(&min_bound, &max_bound);

    for subframe in sub_frames {
        let min_pixels = convert_geo_to_raster(&main_frame, &subframe.min_bound);

        for y in 0..subframe.raster_height {
            for x in 0..subframe.raster_width {
                main_frame.grid[y+min_pixels.y][x+min_pixels.x] = subframe.grid[y][x];
            }
        }
    };

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

        assert_eq!(partition_bounds(&min, &max).len(), 12);
    }

    #[test]
    fn test_partition_bounds_creates_one_subframe() {
        let min = GeoPoint{longitude:-3.8,latitude:50.0};
        let max = GeoPoint{longitude:-3.2, latitude:50.6};

        assert_eq!(partition_bounds(&min, &max).len(), 1);
    }
}