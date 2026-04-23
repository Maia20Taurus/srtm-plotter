use srtm_plotter::{SrtmFrame, geo::*};
use plotters::prelude::*;
use plotters_backend::BackendColor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let min_bound = GeoPoint{
        latitude:50.57465537368716-0.2, longitude:-3.5075965576970947-0.2
    };
    let max_bound = GeoPoint{
        latitude:min_bound.latitude+1.0, longitude:min_bound.longitude+1.0
    };
    let frame = SrtmFrame::new(&min_bound, &max_bound);

    let res_x = frame.raster_width as u32;
    let res_y = frame.raster_height as u32;

    let mut backend = BitMapBackend::new(
        "images/test.png",
        (res_x, res_y)
    );

    for y in 0..res_y {
        for x in 0..res_x {
            let pixel_x = (x as f64 * (frame.raster_width - 1) as f64/res_x as f64) as usize;
            let pixel_y = (y as f64 * (frame.raster_height - 1) as f64/res_y as f64) as usize;

            let pixel = RasterPoint{x:x as usize,y:y as usize};

            let elevation = frame.get_elevation_at_pixel(&pixel);

            let color = BackendColor {
                alpha: 1.0,
                rgb: (elevation as u8, elevation as u8, elevation as u8)
            };

            backend.draw_pixel((pixel_x as i32, pixel_y as i32), color)?;

        }
    }

    backend.present()?;

    Ok(())
}