use srtm_plotter::{SrtmFrame, geo::*};

fn main() {
    let min_bound = GeoPoint{
        longitude:-3.5347112815644306,
        latitude:50.73705448837367,
    };
    let max_bound = GeoPoint{
        longitude:min_bound.longitude+0.01,
        latitude:min_bound.latitude+1.01
    };
    let frame = SrtmFrame::new(&min_bound, &max_bound);
    println!("{:?}", frame.grid);
}