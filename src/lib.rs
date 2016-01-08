use std::vec::Vec;

const PI: f64 = 3.141592;


// represents a location in decimal degrees format
struct Location {
  lat: f64,
  lon: f64
}

// Degrees, Minutes, Seconds
struct DMS {
  d: i32,
  m: i32,
  s: i32
}

impl DMS {
  fn to_decimal(&self) -> f64 {
    let dd = (self.d as f64).abs();
    let mm = self.m as f64;
    let ss = self.s as f64;
    let mut ret = dd + mm/60.0 + ss/3600.0;
    if ((self.d as f64) < (0 as f64)) {
      ret = (0 as f64) - ret;
    }
    ret
  }
}

impl ToString for DMS {
  fn to_string(&self) -> String {
    format!("{}° {}' {}\"", self.d, self.m, self.s)
  }
}

impl Location {

    // construct a new location from decimal degrees
    fn new(_lat: f64, _lon: f64) -> Location {
        Location { lat: _lat, lon: _lon }
    }

    fn calc_bearing_to(&self, dest: &Location) -> f64 {
        let start_lat = radians(self.lat);
        let start_long = radians(self.lon);
        let dest_lat = radians(dest.lat);
        let dest_long = radians(dest.lon);
        let mut delta_long = dest_long - start_long;

        let delta_phi = ((dest_lat/2.0+PI/4.0).tan()/(start_lat/2.0+PI/4.0).tan()).ln();

        if delta_long.abs() > PI {
          if delta_long > 0.0 {
             delta_long = -(2.0 * PI - delta_long);
         } else {
             delta_long = (2.0 * PI + delta_long);
         }
        }

        return (degrees(delta_long.atan2(delta_phi)) + 360.0) % 360.0;
    }
}

impl ToString for Location {
  fn to_string(&self) -> String {
    format!("{}, {}", self.lat, self.lon)
  }
}

fn radians(n: f64) -> f64 {
    n * (PI / 180.0)
}

fn degrees(n: f64) -> f64 {
    n * (180.0 / PI)
}

#[test]
fn calc_bearing_boulder_to_dia() {

  // 39.8617° N, 104.6731° W
  let dia = Location::new(39.8617, -104.6731);

  // 40.0274° N, 105.2519° W
  let boulder = Location::new(40.0274, -105.2519);

  assert_eq!("110.48", format!("{:.*}", 2, boulder.calc_bearing_to(&dia)));

}

#[test]
fn convert_dms_to_decimal() {
  let dia = Location::new(
      DMS { d: 39, m: 51, s: 42 }.to_decimal(),
      DMS { d: -104, m: 40, s: 22 }.to_decimal()
  );

  assert_eq!("39.861666666666665, -104.67277777777778", dia.to_string());
}

#[test]
fn test_sparkfun_route() {

    let mut route: Vec<Location> = Vec::new();
    route.push(Location::new(40.0906963, -105.185844));
    route.push(Location::new(40.0908317, -105.185734));
    route.push(Location::new(40.0910061, -105.1855154));

    //TODO: need to confirm that these bearings are actually correct
    assert_eq!("31.86", format!("{:.*}", 2, &route[0].calc_bearing_to(&route[1])));
    assert_eq!("43.80", format!("{:.*}", 2, &route[1].calc_bearing_to(&route[2])));

}
