<p align="center">
    <img src="design/logo/syunit_logo_titled.svg" width="50%" />
</p>

-----------------------------

A small library that contains some basic units to help structuring kinematics and robotic programming in rust. The library uses rusts *tuple structs* to create a zero-overhead and compile-time checking of correct unit, variable and function usage.

## Quick introduction

In many functions for kinematics and robotics, it sometimes becomes unclear which type of unit is desired to be used, especially when it
comes to distances. 

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

Even if most code is not as horribly documented and uses such terrible names, nothing stops a developer from accidently plugging in an absolute distance into a function that takes a relative one. Here comes this library into play:

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

// RelDist => Relative distance
// AbsPos => Absolute distance
//
// Naming choice will be explained later
```

Each unit is represented by a 32bit float enclosed into a *tuple struct*. Why these units are helpful not only for documentation is explained in the flowing chapters:

## Creation and conversion

As rust always prefers implicit syntax, so does this library. The unit types cannot be converted back to a `f32` without calling `into()`.

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

## Naming

As the units are all named after their purpose, the context of functions, their parameters and other variables becomes clear easier. However the library does *not* differentiate between linear and rotary movement in terms of naming.

However there are two units for distances with different names:

- `AbsPos`: Represents an absolute distance *component angle/distance*
- `RelDist`: Represents a relative distance

## Operations and automatic type evaluation

Especially with distances, a lot of operations between them are restricted, as they would fail to make any sense. For example an `AbsPos` distance cannot be added to another `AbsPos` distance, as it does not make any sense to add two absolute distances. However a `RelDist` distance can be added to an `AbsPos` to extend/shorten said distance. 

```rust
use syunit::prelude::*;

let abs_pos = PositionMM(2.0);
let rel_dist = Millimeters(1.0);

assert_eq!(abs_pos + rel_dist, PositionMM(3.0));
assert_eq!(abs_pos - rel_dist, PositionMM(1.0));
```

Also it is for example possible to subtract two absolute distances, which gives the relative `RelDist` distance between them.

```rust
use syunit::prelude::*;

assert_eq!(PositionMM(5.0) - PositionMM(3.0), Millimeters(2.0));
```

A very special unit is `Time`, dividing or multipling by it often changes units.

```rust
use syunit::prelude::*;

// Travelling a distance of 6mm in 2 seconds gives a velocity of 3mm/s
assert_eq!(Millimeters(6.0) / Seconds(2.0), MMPerSecond(3.0));
// Accelerating to a velocity of 3mm/s in 2 seconds gives an acceleration of 1.5mm/s^2
assert_eq!(MMPerSecond(3.0) / Seconds(2.0), MMPerSecond2(1.5));
// Travelling with 3mm/s for 3 seconds gives a total distance of 9mm
assert_eq!(MMPerSecond(3.0) * Seconds(3.0), Millimeters(9.0));
```

## Specific units - Metric and Imperial

In addition to these universal units (like `Velocity`, `Acceleration` ...), the library also includes metric and imperial units and conversions between them.

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

Please feel free to create issues on the [github repo](https://github.com/SamuelNoesslboeck/syunit) or contact me directly.