use srtm_plotter::{SrtmFrame, geo::*};

fn main() {
    let min_bound = GeoPoint{
        longitude:-3.5347112815644306,
        latitude:50.73705448837367,
    };
    let max_bound = GeoPoint{
        longitude:min_bound.longitude+1.5,
        latitude:min_bound.latitude+1.5
    };
    let frame = SrtmFrame::new(&min_bound, &max_bound);

    let point = GeoPoint {
        longitude: -3.0,
        latitude: 51.0
    };
    let elevation = frame.get_elevation_at_point(&point);
    println!("{}", elevation);
}