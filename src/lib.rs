
// represents a location in decimal degrees format
struct LocationDecimal {
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


// represents a location in Degrees, Minutes, Seconds format
struct LocationDMS {
  lat: DMS,
  lon: DMS
}

impl LocationDMS {
  fn to_decimal(&self) -> LocationDecimal {
    LocationDecimal { lat: self.lat.to_decimal(), lon: self.lon.to_decimal() }
  }
}


impl ToString for DMS {
  fn to_string(&self) -> String {
    format!("{}° {}' {}\"", self.d, self.m, self.s)
  }
}

impl ToString for LocationDMS {
  fn to_string(&self) -> String {
    format!("{}, {}", self.lat.to_string(), self.lon.to_string())
  }
}

impl ToString for LocationDecimal {
  fn to_string(&self) -> String {
    format!("{}, {}", self.lat, self.lon)
  }
}

//fn convert_to_decimal(l: &LocationDMS) -> LocationDecimal {
//  //TODO: complete this
//  LocationDecimal { lat: DMS { d: l.lat.trunc(), m: 0, s: 0} , lon: DMS { d: l.lon.trunc(), m: 0, s: 0 } }
//}

#[test]
fn it_works() {

  // 39.8617° N, 104.6731° W
  let dia = LocationDecimal { lat: 39.8617, lon: -104.6731 };

  // 40.0274° N, 105.2519° W
  let boulder = LocationDecimal { lat: 40.0274, lon: -105.2519 };

  assert_eq!("39.8617, -104.6731", dia.to_string());

}

#[test]
fn convert_dms_to_decimal() {
  let diaDMS = LocationDMS { lat: DMS { d: 39, m: 51, s: 42 }, lon: DMS { d: -104, m: 40, s: 22 } };
  assert_eq!("39.861666666666665, -104.67277777777778", diaDMS.to_decimal().to_string());
}

