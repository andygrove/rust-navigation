
struct Location {
  lat: f64,
  lon: f64
}

impl ToString for Location {
  fn to_string(&self) -> String {
    format!("{}, {}", self.lat, self.lon)
  }
}


#[test]
fn it_works() {

  // 39.8617° N, 104.6731° W
  let dia = Location { lat: 39.8617, lon: -104.6731 };

  // 40.0274° N, 105.2519° W
  let boulder = Location { lat: 40.0274, lon: -105.2519 };

  println!("DIA is at {}", dia.to_string());
}
