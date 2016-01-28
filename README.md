# rust-navigation

[![Version](https://img.shields.io/crates/v/navigation.svg)](https://crates.io/crates/navigation)

This is a very simple navigation crate intended for use with autonomous vehicles. It simply provides functions to calculate the required bearing to navigate between two waypoints based on latitude and longtitude.

Two methods are provided - an accurate algorithm and a faster estimation method based on simple 2D trigonometry that does not take into account the fact that the earth is round. The latter approach is much lighter computationally and accurate enough for calculating bearings between points in a small area, such as a parking lot.

## Example

Which direction should I go to get to Denver International Airport from Boulder, CO?

```
let dia = Location::new(39.8617, -104.6731);
let boulder = Location::new(40.0274, -105.2519);
let bearing = boulder.calc_bearing_to(&dia);
```
