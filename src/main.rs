use srtm_plotter::{SrtmFrame, geo::*};
use plotters::prelude::*;
use plotters_backend::BackendColor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let min_bound = GeoPoint{
        latitude:50.57312599959167, longitude:-3.681166929507134
    };
    let max_bound = GeoPoint{
        latitude:50.810654922812375, longitude:-3.406491509839958
    };
    let frame = SrtmFrame::new(&min_bound, &max_bound);

    let point = GeoPoint {
        longitude: -3.0,
        latitude: 51.0
    };
    //let elevation = frame.get_elevation_at_point(&point);
    //println!("{}", elevation);

    let mut backend = BitMapBackend::new(
        "images/test.png",
        (600, 500)
    );

    for y in 0..frame.raster_height {
        for x in 0..frame.raster_width {
            let elevation = frame.get_elevation_at_pixel(x, y);

            let x_val = ((x as f64)/(frame.raster_width as f64) * 600.0) as i32;
            let y_val = ((y as f64)/(frame.raster_height as f64) * 500.0) as i32;

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