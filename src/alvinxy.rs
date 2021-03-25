//! This is module converts between GPS coordinates in decimal form to a local coordinate structure per the AlvinXY algorithm as laid out in the paper below.
//!
//! `Murphy, Chris & Singh, Hanumant. (2010). Rectilinear coordinate frames for Deep sea navigation. 2010 IEEE/OES Autonomous Underwater Vehicles, AUV 2010. 1 - 10. 10.1109/AUV.2010.5779654.`
//!
//! This implementation has been used to generate waypoints for an autonomous underwater vehicle mission in a freshwater resevoir

use std::f64;

#[derive(Clone, Copy, Debug)]
pub struct LocalCoor {
    pub x: f64,
    pub y: f64,
}

use crate::Location;

impl LocalCoor {
    /// Create a new coordinate in the local reference frame
    pub fn new(x: f64, y: f64) -> Self {
        LocalCoor { x, y }
    }

    /// Rotate a coordinate around the origin by an angle supplied in degrees
    pub fn rotate_local(&mut self, theta: f64) {
        let theta = theta.to_radians();
        self.x = (self.x * theta.cos()) - (self.y * theta.sin());
        self.y = (self.x * theta.sin()) + (self.y * theta.cos());
    }
}

/// Convert a GPS `Location` coordinate to a local `LocalCoor` coordinate frame
pub fn latlon2xy(coordinate: Location, origin: Location) -> LocalCoor {
    let x = (coordinate.lon - origin.lon) * mdeglon(origin.lat);
    let y = (coordinate.lat - origin.lat) * mdeglat(origin.lat);
    LocalCoor::new(x, y)
}

/// Converts from the local `LocalCoor` xy to global `Location` Coordinate frame
pub fn xy2latlon(coordinate: LocalCoor, origin: Location) -> Location {
    let lon = coordinate.x / mdeglon(origin.lat) + origin.lon;
    let lat = coordinate.y / mdeglat(origin.lat) + origin.lat;
    Location { lat, lon }
}

#[inline]
fn mdeglon(lat0: f64) -> f64 {
    let lat0rad = lat0.to_radians();
    111415.13 * lat0rad.cos() - (94.55 * (3.0 * lat0rad).cos()) - (0.12 * (5.0 * lat0rad).cos())
}

#[inline]
fn mdeglat(lat0: f64) -> f64 {
    let lat0rad = lat0.to_radians();
    111132.09 - (566.05 * (2.0 * lat0rad).cos()) + (1.20 * (4.0 * lat0rad).cos())
        - (0.002 * (6.0 * lat0rad).cos())
}

// This test was used to verify the coordinate offsets for an AUV mission in a Californian freshwater resevoir
#[test]
fn reservoir_coordinates() {
    let origin = Location::new(34.589000, -119.966000);
    let o = latlon2xy(Location::new(34.589000, -119.966000), origin); // Making sure the origin in (0.0,0.0)
    println!("The origin is at: ({},{})", o.x, o.y);
    assert!((o.x == 0.0) && (o.y == 0.0));
    // Directly above/North
    let n = latlon2xy(Location::new(34.59000, -119.966000), origin);
    println!("nx,ny = ({},{})", n.x, n.y);
    assert!(n.y > o.y);
    // Directly below/South
    let s = latlon2xy(Location::new(34.58800, -119.966000), origin);
    println!("nx,ny = ({},{})", s.x, s.y);
    assert!(s.y < o.y);
    // Directly left/West (longitude DECREASES going West)
    let w = latlon2xy(Location::new(34.589000, -119.967000), origin);
    println!("wx,wy = ({},{})", w.x, w.y);
    assert!(w.x < o.x);
    // Directly right/East (latitude INCREASES going East)
    let e = latlon2xy(Location::new(34.589000, -119.96500), origin);
    println!("ex,ey = ({},{})", e.x, e.y);
    assert!(e.x > o.x);
}
