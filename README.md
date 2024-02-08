# syunit

A small library that contains some basic units to help structuring kinematics and robotic programming in rust. The library uses rusts *tuple structs* to create a zero-overhead and compile-time checking of correct unit, variable and function usage.

## Quick introduction

In many functions for kinematics and robotics, it sometimes becomes unclear which type of unit is desired to be used, especially when it
comes to distances. 

```rust
/// Relative movement
fn move_distance(dist : f32, speed : f32) {
    // ...
}

/// Absolute movement
fn move_to_distance(dist : f32, speed : f32) {
    // ...
}
```

Even if most code is not as horribly documented and uses such terrible names, nothing stops a developer from accidently plugging in an absolute distance into a function that takes a relative one. Here comes this library into play:

```rust
use syunit::*;

/// Relative movement
fn move_distance(dist : Delta, speed : Velocity) {
    // ...
}

/// Absolute movement
fn move_to_distance(dist : Gamma, speed : Velocity) {
    // ...
}

// Delta => Relative distance
// Gamma => Absolute distance
//
// Naming choice will be explained later
```

Each unit is represented by a 32bit float enclosed into a *tuple struct*. Why these units are helpful not only for documentation is explained in the flowing chapters:

### Creation and conversion

As rust always prefers implicit syntax, so does this library. The unit types cannot be converted back to a `f32` without calling `into()`.

```rust ,compile_fail
use syunit::*;

fn requires_f32(value : f32) {
    // ...
}

fn requires_velocity(value : Velocity) {
    // ...
}

// Every unit is created by the tuple struct constructor using a `f32` value
let gamma = Gamma(10.0);

requires_f32(gamma);
// error[E0308]: mismatched types
// | requires_f32(gamma) // ERROR! => Type `Gamma` cannot be used as `f32`
// | ------------ ^^^^^ expected `f32`, found `Gamma`
// | |
// | arguments to this function are incorrect
// |

requires_velocity(gamma);
// error[E0308]: mismatched types
// |
// | requires_f32(gamma);
// | ------------ ^^^^^ expected `Velocity`, found `Gamma`
// | |
// | arguments to this function are incorrect
// |
```

### Naming

As the units are all named after their purpose, the context of functions, their parameters and other variables becomes clear easier. However the library does *not* differentiate between linear and rotary movement in terms of naming.

However there are three units for distances with different names:

- `Gamma`: Represents an absolute distance in the actuators "perspective", often refered to as *component angle/distance*
- `Phi`: Represents an absolute distance in the machines "perspective", often refered to as *mathematical angle/distance* in a lot of documentations. This angle is for example used to describe the rotation of a robot joint, where the `Gamma` angle has an offset compared to the `Phi` angle.
- `Delta`: Represents a relative distance

### Operations and automatic type evaluation

Especially with distances, a lot of operations between them are restricted, as they would fail to make any sense. For example a `Gamma` distance cannot be added with either a `Phi` or another `Gamma` distance, as it does not make any sense to add two absolute distances. However a `Delta` distance can be added to a `Gamma` or `Phi` distance to extend/shorten said `Gamma` or `Phi` distance. 

```rust
use syunit::*;

let gamma = Gamma(2.0);
let phi = Phi(4.0);
let delta = Delta(1.0);

assert_eq!(gamma + delta, Gamma(3.0));
assert_eq!(phi + delta, Phi(5.0));
```

Also it is for example possible to subtract two absolute distances, which gives the relative `Delta` distance between them.

```rust
use syunit::*;

assert_eq!(Gamma(5.0) - Gamma(3.0), Delta(2.0));
```

A very special unit is `Time`, dividing or multipling by it often changes units.

```rust
use syunit::*;

// Travelling a distance of 6mm in 2 seconds gives a velocity of 3mm/s
assert_eq!(Delta(6.0) / Time(2.0), Velocity(3.0));
// Accelerating to a velocity of 3mm/s in 2 seconds gives an acceleration of 1.5mm/s^2
assert_eq!(Velocity(3.0) / Time(2.0), Acceleration(1.5));
// Travelling with 3mm/s for 3 seconds gives a total distance of 9mm
assert_eq!(Velocity(3.0) * Time(3.0), Delta(9.0));
```

### Physical background

Each unit of course represents a physical unit, in almost all cases their standardized values. Only difference is distance, it is represented by *millimeters*. Meaning velocity becomes *millimeters per second*, acceleration becomes *millimeters per second squared* ...

## Issues and improvements

Please feel free to create issues on the [github repo](https://github.com/SamuelNoesslboeck/syunit) or contact me directly.