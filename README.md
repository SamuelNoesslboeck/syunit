<p align="center">
    <img src="design/logo/syunit_logo_titled.svg" width="50%" />
</p>

-----------------------------

A small library that contains some basic units to help structuring kinematics and robotic programming in rust. The library uses rusts *tuple structs* to create a compile-time checking of correct unit, variable and function usage.

## Quick introduction

In many functions for kinematics and robotics, it sometimes becomes unclear which type of unit is desired to be used.

```rust
/// Relative movement
fn move_distance(dist : f32, vel : f32) {
    // ...
}

/// Absolute movement
fn move_to_position(pos : f32, vel : f32) {
    // ...
}
```

Even if most code is not as horribly documented, nothing stops a developer from accidently plugging in an absolute distance into a function that takes a relative one. Here comes this library into play:

```rust
use syunit::prelude::*;

/// Relative movement
fn move_distance(dist : Radians, vel : RadPerSecond) {
    // ...
}

/// Absolute movement
fn move_to_position(pos : PositionRad, vel : RadPerSecond) {
    // ...
}
```

Each unit is represented by a `f32` enclosed into a *tuple struct*, making them simple but also their own type!

Why these units are helpful not only for documentation is explained in the flowing chapters:


### Explicit syntax

As rust always prefers explicit syntax, so does this library. The unit types cannot be converted back to a `f32` without calling `into()`.

```rust ,compile_fail
use syunit::prelude::*;

fn requires_f32(value : f32) {
    // ...
}

fn requires_velocity(value : MMPerSecond) {
    // ...
}

// Every unit is created by the tuple struct constructor using a `f32` value
let abs_pos = PositionMM(10.0);

requires_f32(abs_pos);
// error[E0308]: mismatched types
// |
// | requires_f32(abs_pos) // ERROR! => Type `PositionMM` cannot be used as `f32`
// | ------------ ^^^^^ expected `f32`, found `PositionMM`
// | |
// | arguments to this function are incorrect
// |

requires_velocity(abs_pos);
// error[E0308]: mismatched types
// |
// | requires_f32(abs_pos);
// | ------------ ^^^^^ expected `MMPerSecond`, found `PositionMM`
// | |
// | arguments to this function are incorrect
// |
```


### Operations and automatic type evaluation

The library comes with a lot of implementations in addition to the units, making it possible to do a lot of operations with these units and letting the compiler automatically evaluate the resulting units for you

```rust
use syunit::prelude::*;

// Radial / Linear
assert_eq!(RadPerSecond(4.0) * Millimeters(2.0), MMPerSecond(8.0));
assert_eq!(RadPerSecond2(-3.0) * Millimeters(2.0), MMPerSecond2(-6.0));

// Seconds / Hertz
assert_eq!(Hertz(4.0), 1.0 / Seconds(0.25));
assert_eq!(1.0 / Hertz(5.0), Seconds(0.2));

// Time to build up speed
assert_eq!(MMPerSecond(6.0) / MMPerSecond2(2.0), Seconds(3.0));

// Forces
assert_eq!(NewtonMeters(-5.0) / KgMeter2(2.0), RadPerSecond2(-2.5));
assert!((Newtons(3.0) / Kilogramms(1.5) - MMPerSecond2(2000.0)).abs().0 < 0.001);  // Automatic conversion

// ...
```

Another helpful unit type are `Positions`, they help differentiating between *absolute* and *relative* distances.

```rust
use syunit::prelude::*;

// Difference between two positions is a relative distance
assert_eq!(PositionMM(5.0) - PositionMM(3.0), Millimeters(2.0));

// Radial position math
assert_eq!(PositionRad(3.0) + Radians(2.0), PositionRad(5.0));
assert_eq!(PositionRad(3.0) - PositionRad(2.0), Radians(1.0)); 
```

A very special unit is `Seconds`, dividing or multipling by it often changes units.

```rust
use syunit::prelude::*;

// Travelling a distance of 6mm in 2 seconds gives a velocity of 3mm/s
assert_eq!(Millimeters(6.0) / Seconds(2.0), MMPerSecond(3.0));
// Accelerating to a velocity of 3mm/s in 2 seconds gives an acceleration of 1.5mm/s^2
assert_eq!(MMPerSecond(3.0) / Seconds(2.0), MMPerSecond2(1.5));
// Travelling with 3mm/s for 3 seconds gives a total distance of 9mm
assert_eq!(MMPerSecond(3.0) * Seconds(3.0), Millimeters(9.0));
```

### Unitsets

There is also a tool for defining functions in a more general way, with the help of `UnitSets`!

```rust
use syunit::prelude::*;

fn get_distance<U : UnitSet>(vel : U::Velocity, time : U::Time) -> U::Distance {
    vel * time      // Compiler automatically checks if the types match
}

fn time_for_dist_accelerating<U : UnitSet>(distance : U::Distance, vel_start : U::Velocity, vel_end : U::Velocity) -> U::Time {
    let vel_avg = (vel_start + vel_end) / 2.0;
    distance / vel_avg
}

assert_eq!(get_distance::<MetricMM>(MMPerSecond(4.0), Seconds(2.0)), Millimeters(8.0));     // Using linear metric mm
assert_eq!(time_for_dist_accelerating::<Rotary>(Radians(3.0), RadPerSecond(2.0), RadPerSecond(4.0)), Seconds(1.0));     // Using rotary units
```

### Metric and Imperial

The library also includes imperial units and conversions between them.

```rust
use syunit::prelude::*;
use syunit::imperial::*;

let millimeters = Millimeters(25.4);

assert_eq!(Meters(1.0), Millimeters(1000.0).into());
assert_eq!(Inches(1.0), Millimeters(25.4).into());
```

## `serde` implementation

All the units implement `serde::Serialize` and `serde::Deserialize` if the "serde" feature is enabled, which is the case by default. 

## Issues and improvements

Please feel free to create issues on the [github repo](https://github.com/SamuelNoesslboeck/syunit)!