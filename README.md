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

## AlvinXY submodule

The `alvinxy` submodule adds the ability to switch between a local and global coordinate frame according to the
```
Murphy, Chris & Singh, Hanumant. (2010). Rectilinear coordinate frames for Deep sea navigation. 2010 IEEE/OES Autonomous Underwater Vehicles, AUV 2010. 1 - 10. 10.1109/AUV.2010.5779654.`
```

### AlvinXY mission planning example

A mission planned for a vehicle using this coordinate plan might look like the following, where the final GPS coordinates could be exported to a mission file or use directly by a control system. This implementation has been used for mission planning of an autonomous underwater vehicle in a freshwater reservoir.

```
fn main() {
    let origin = Location::new(34.589,-119.96472);
    // Writing the vehicle coordinate sequence and push them to vector
    let mut wt_list: Vec<LocalCoor> = Vec::new();
    let mut b = LocalCoor::new(-900.0,0.0);
    let mut soff = LocalCoor::new(-900.,-100.);
    let mut son = LocalCoor::new(-900.,100.);
    wt_list.push(b);
    wt_list.push(soff);
    wt_list.push(son);

    for wpt in &mut wt_list {
        wpt.rotate_local(15.0);
        println!("List of waypoints after rotation and conversion: {:?}",xy2latlon(*wpt,origin));
    }
}
```
