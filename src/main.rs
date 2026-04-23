use srtm_plotter::{SrtmFrame, geo::*};
use plotters::prelude::*;
use plotters_backend::BackendColor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let min_bound = GeoPoint{
        latitude:50.1, longitude:-3.9
    };
    let max_bound = GeoPoint{
        latitude:min_bound.latitude+0.33, longitude:min_bound.longitude+0.33
    };
    let frame = SrtmFrame::new(&min_bound, &max_bound);

    let point = GeoPoint {
        longitude: -3.0,
        latitude: 51.0
    };
    //let elevation = frame.get_elevation_at_point(&point);
    //println!("{}", elevation);

    let res_x = 1000;
    let res_y = 1000;

    let mut backend = BitMapBackend::new(
        "images/test.png",
        (res_x, res_y)
    );

    for y in 0..frame.raster_height {
        for x in 0..frame.raster_width {
            let elevation = frame.get_elevation_at_pixel(x, y);

            let x_val = ((x as f64)/(frame.raster_width as f64) * res_x as f64) as i32;
            let y_val = ((y as f64)/(frame.raster_height as f64) * res_y as f64) as i32;

            let color = BackendColor {
                alpha: 1.0,
                rgb: (elevation as u8, elevation as u8, elevation as u8)
            };

            backend.draw_pixel((x_val, y_val), color)?;
        }
    }

    backend.present()?;

    Ok(())
}