extern crate rand;
pub mod alvinxy;

use std::f64::consts::PI;

/// parses an NMEA string (degrees decimal minutes) such as "3953.4210" into (39, 53.4210) and
/// then to 39 + 53.4210/60
pub fn parse_nmea(s: &str) -> Result<f64, String> {
    match String::from(s).parse::<f64>() {
        Ok(n) => Ok(((n / 100_f64) as u32) as f64 + ((n % 100_f64) / 60_f64)),
        Err(e) => Err(format!("Failed to parse: {}", e)),
    }
}

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

// represents a location in decimal degrees format
#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

impl Location {
    pub fn parse_nmea(lat: &str, lat_dir: &str, lon: &str, lon_dir: &str) -> Result<Self, String> {
        let lat_dir = Location::parse_direction(lat_dir)?;
        let lat_mult = match lat_dir {
            Direction::North => Ok(1_f64),
            Direction::South => Ok(-1_f64),
            _ => Err(format!("Invalid latitude direction {:?}", lat_dir)),
        }?;
        let lat = parse_nmea(lat)? * lat_mult;

        let lon_dir = Location::parse_direction(lon_dir)?;
        let lon_mult = match lon_dir {
            Direction::East => Ok(1_f64),
            Direction::West => Ok(-1_f64),
            _ => Err(format!("Invalid longitude direction {:?}", lon_dir)),
        }?;
        let lon = parse_nmea(lon)? * lon_mult;

        Ok(Location { lat, lon })
    }

    pub fn parse_direction(d: &str) -> Result<Direction, String> {
        match d {
            "N" => Ok(Direction::North),
            "S" => Ok(Direction::South),
            "E" => Ok(Direction::East),
            "W" => Ok(Direction::West),
            _ => Err(format!("Invalid direction '{}'", d)),
        }
    }
}

// Degrees, Minutes, Seconds
pub struct DMS {
    d: i32,
    m: i32,
    s: i32,
}

impl DMS {
    pub fn to_decimal(&self) -> f64 {
        let dd = (self.d as f64).abs();
        let mm = self.m as f64;
        let ss = self.s as f64;
        let mut ret = dd + mm / 60.0 + ss / 3600.0;
        if (self.d as f64) < (0 as f64) {
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
    pub fn new(_lat: f64, _lon: f64) -> Location {
        Location {
            lat: _lat,
            lon: _lon,
        }
    }

    pub fn set(&mut self, lat: f64, lon: f64) {
        self.lat = lat;
        self.lon = lon;
    }

    pub fn calc_bearing_to(&self, dest: &Location) -> f64 {
        let start_lat = radians(self.lat);
        let start_long = radians(self.lon);
        let dest_lat = radians(dest.lat);
        let dest_long = radians(dest.lon);
        let mut delta_long = dest_long - start_long;

        let delta_phi =
            ((dest_lat / 2.0 + PI / 4.0).tan() / (start_lat / 2.0 + PI / 4.0).tan()).ln();

        if delta_long.abs() > PI {
            if delta_long > 0.0 {
                delta_long = -(2.0 * PI - delta_long);
            } else {
                delta_long += 2.0 * PI;
            }
        }

        (degrees(delta_long.atan2(delta_phi)) + 360.0) % 360.0
    }

    /** experimental cheaper method of calculating bearing */
    pub fn estimate_bearing_to(&self, dest: &Location, lat_size: f64, lon_size: f64) -> f64 {
        let lat_delta = (dest.lat - self.lat) * lat_size;
        let lon_delta = (dest.lon - self.lon) * lon_size;

        // println!("delta: lat={}, lon={}", lat_delta, lon_delta);

        let ax = lon_delta.abs();
        let ay = lat_delta.abs();

        let angle: f64 = 180.0 / PI
            * if ax > ay {
                (ay / ax).atan()
            } else {
                (ax / ay).atan()
            };

        // println!("angle = {}", angle);

        let bearing: f64 = if lon_delta > 0.0 {
            if lat_delta > 0.0 {
                if ax > ay {
                    90.0 - angle
                } else {
                    angle
                }
            } else {
                if ax > ay {
                    90.0 + angle
                } else {
                    180.0 - angle
                }
            }
        } else {
            if lat_delta > 0.0 {
                if ax > ay {
                    270.0 + angle
                } else {
                    360.0 - angle
                }
            } else {
                if ax > ay {
                    270.0 - angle
                } else {
                    180.0 + angle
                }
            }
        };

        bearing
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
fn test_estimation_accuracy() {
    let lon_min = -105.18591;
    let lon_max = -105.18467;
    let lat_min = 40.09027;
    let lat_max = 40.09145;

    let mut ok = true;

    for _ in 0..100000 {
        // create two random points on map
        let l1 = Location::new(
            lat_min + rand::random::<f64>() * (lat_max - lat_min),
            lon_min + rand::random::<f64>() * (lon_max - lon_min),
        );

        let l2 = Location::new(
            lat_min + rand::random::<f64>() * (lat_max - lat_min),
            lon_min + rand::random::<f64>() * (lon_max - lon_min),
        );

        let bearing = l1.calc_bearing_to(&l2);
        // let estimate = l1.estimate_bearing_to(&l2, 1.0, 1.0);
        let estimate = l1.estimate_bearing_to(&l2, 69.0, 53.0);
        let diff = (bearing - estimate).abs();

        println!(
            "({}, {}) -> ({}, {}): bearing={} estimate={} diff={} [{}]",
            format!("{:.*}", 6, l1.lat),
            format!("{:.*}", 6, l1.lon),
            format!("{:.*}", 6, l2.lat),
            format!("{:.*}", 6, l2.lon),
            format!("{:.*}", 1, bearing),
            format!("{:.*}", 1, estimate),
            format!("{:.*}", 1, diff),
            if diff < 1.0 { "OK" } else { "FAIL" }
        );

        if diff > 1.0 {
            ok = false;
            break;
        }
    }

    assert!(ok);
}

#[test]
fn calc_bearing_boulder_to_dia() {
    // 39.8617° N, 104.6731° W
    let dia = Location::new(39.8617, -104.6731);

    // 40.0274° N, 105.2519° W
    let boulder = Location::new(40.0274, -105.2519);

    assert_eq!("110.48", format!("{:.*}", 2, boulder.calc_bearing_to(&dia)));
    assert_eq!(
        "110.44",
        format!("{:.*}", 2, boulder.estimate_bearing_to(&dia, 69.0, 53.0))
    );
}

#[test]
fn convert_dms_to_decimal() {
    let dia = Location::new(
        DMS {
            d: 39,
            m: 51,
            s: 42,
        }
        .to_decimal(),
        DMS {
            d: -104,
            m: 40,
            s: 22,
        }
        .to_decimal(),
    );

    assert_eq!("39.861666666666665, -104.67277777777778", dia.to_string());
}

#[test]
fn test_sparkfun_route() {
    let mut route: Vec<Location> = Vec::new();
    route.push(Location::new(40.0906963, -105.185844));
    route.push(Location::new(40.0908317, -105.185734));
    route.push(Location::new(40.0910061, -105.1855154));

    // TODO: need to confirm that these bearings are actually correct
    assert_eq!(
        "31.86",
        format!("{:.*}", 2, &route[0].calc_bearing_to(&route[1]))
    );
    assert_eq!(
        "43.80",
        format!("{:.*}", 2, &route[1].calc_bearing_to(&route[2]))
    );
}

#[test]
fn test_sparkfun_route_2() {
    let mut route: Vec<Location> = Vec::new();
    route.push(Location::new(40.09069, -105.18585));
    route.push(Location::new(40.09128, -105.18517));

    // TODO: need to confirm that these bearings are actually correct
    assert_eq!(
        "41.40",
        format!("{:.*}", 2, &route[0].calc_bearing_to(&route[1]))
    );
}

#[test]
fn test_parse_nmea() {
    assert_eq!(
        "101.6971",
        format!("{:.*}", 4, parse_nmea("10141.82531").unwrap())
    );
}
