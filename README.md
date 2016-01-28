# rust-navigation

[![Version](https://img.shields.io/crates/v/navigation.svg)](https://crates.io/crates/navigation)

This is a very simple navigation crate intended for use with autonomous vehicles. It simply provides functions to calculate the required bearing to navigate between two waypoints based on latitude and longtitude.

Two methods are provided - an accurate algorithm and a faster estimation method based on simple 2D trigonometry that does not take into account the fact that the earth is round. The latter approach is much lighter computationally and accurate enough for calculating bearings between points in a small area, such as a parking lot.

## Example

Which direction should I go to get to Denver International Airport from Boulder, CO?

```
let dia     = Location::new(39.8617, -104.6731);
let boulder = Location::new(40.0274, -105.2519);
```

### Accurate calculation

```
let bearing = boulder.calc_bearing_to(&dia); // results in 110.48
```

### Fast calculation

```
let bearing = boulder.estimate_bearing_to(&dia); // results in 110.44
```

### Sample output from unit test comparing the two methods

```
(40.091306, -105.185494) -> (40.090405, -105.185087): bearing=160.9 estimate=160.9 diff=0.1 [OK]
(40.090801, -105.185641) -> (40.091439, -105.184771): bearing= 46.2 estimate= 46.3 diff=0.1 [OK]
(40.090960, -105.185871) -> (40.090342, -105.184860): bearing=128.6 estimate=128.5 diff=0.1 [OK]
(40.091311, -105.185128) -> (40.090946, -105.184925): bearing=157.0 estimate=156.9 diff=0.1 [OK]
(40.091150, -105.185586) -> (40.091221, -105.185793): bearing=294.2 estimate=294.1 diff=0.1 [OK]
```
