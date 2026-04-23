Read and display elevation data from the SRTM dataset. Note: Does not currently implement displaying the data
but you may still find use in this library's ability to read from multiple GeoTiffs simultaneously.

Example usage for obtaining elevation data from a specified minimum and maximum boundary:
```rust
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
    println!("{:?}", frame.grid);
}
```
