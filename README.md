# rust-navigation

[![Version](https://img.shields.io/crates/v/navigation.svg)](https://crates.io/crates/navigation)

This is a very simple navigation crate intended for use with autonomous vehicles. It simply provides a way to calculate the required bearing to navigate between two waypoints based on latitude and longtitude.

## Example: Which direction should I get to get to Denver International Airport from Boulder, CO?

```
let dia = Location::new(39.8617, -104.6731);
let boulder = Location::new(40.0274, -105.2519);
let bearing = boulder.calc_bearing_to(&dia);
```
