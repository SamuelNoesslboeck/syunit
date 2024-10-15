#![crate_name = "syunit"]
#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]

use core::f32::consts::PI;
use core::ops::{Add, AddAssign, Div, Sub, SubAssign};
use core::time::Duration;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; 

// Submodules
    mod funcs;
    pub use funcs::*;

    mod macros;

    mod specials;
    pub use specials::*;
// 

// Helper import for local macro definitions
use crate as syunit;

/// General marker trait for all units
pub trait Unit : Into<f32> { 
    /// Creates a new value of this unit using a `f32` value
    fn new(v : f32) -> Self
    where 
        Self : Sized;
}

/// Represents a time
/// 
/// # Unit
/// 
/// - In seconds
/// 
/// ```rust
/// use syunit::*;
/// 
/// // Comparisions
/// assert!(Time(1.0) > Time(-1.0));
/// ```
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Time(pub f32);
basic_unit!(Time);
additive_unit!(Time);

impl Into<Duration> for Time {
    #[inline(always)]
    fn into(self) -> Duration {
        // Negative time fallback
        // if self.0.is_sign_negative() {
        //     self.0 = self.0.abs();
        // }

        Duration::from_secs_f32(self.0)
    }
}

impl Div<Time> for f32 {
    type Output = Velocity;

    #[inline(always)]
    fn div(self, rhs: Time) -> Self::Output {
        Velocity(self / rhs.0)
    }
}

/// Represents a certain factor between 0 and 1
/// 
/// # Unit
/// 
/// - Unitless
/// 
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Factor(f32);

impl Factor {
    // Constants
        /// Maximum
        pub const MAX : Self = Self(1.0);

        /// Half
        pub const HALF : Self = Self(0.5);

        /// Minium
        pub const MIN : Self = Self(0.0);
    // 

    /// Creates a new `Factor`
    /// 
    /// # Panics
    /// 
    /// Panics if the given 'val' is out of bounds (0 <= val <= 1)
    pub fn new(val : f32) -> Self {
        Self::try_new(val)
            .expect("Given value for factor is out of bounds!")
    }

    /// Tries to create a new factor, will be `None` if `val` is not between or equal to 0 and 1
    pub fn try_new(val : f32) -> Option<Self> {
        if (val >= 0.0) & (val <= 1.0) {
            Some(Self(val))
        } else {
            None
        }
    }

    /// Creates a new factor without checking bounds
    /// 
    /// # Unsafe
    /// 
    /// An out of bounds factor might throw up important logic
    /// 
    /// Should only be used for creating literals
    pub const unsafe fn new_unchecked(val : f32) -> Self {
        Self(val)
    }

    /// embedded_hal helper function
    pub fn get_duty(self) -> u16 {
        ((u16::MAX as f32) * self.0) as u16 
    }

    /// embedded_hal helper function
    pub fn get_duty_for(self, max_duty : u16) -> u16 {
        ((max_duty as f32) * self.0) as u16 
    }
}

impl core::ops::Mul<Factor> for Factor {
    type Output = Factor;

    fn mul(self, rhs: Factor) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl core::ops::Mul<Factor> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: Factor) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl core::ops::Mul<Factor> for Time {
    type Output = Time;

    fn mul(self, rhs: Factor) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl core::fmt::Display for Factor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::fmt::Debug for Factor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Factor({})", self.0))
    }
}

/// The `AbsPos` unit represents the absolute position of a component
/// 
/// # Unit
/// 
/// - Can be either radians or millimeters
/// 
/// # Operations
/// 
/// ```rust
/// use syunit::{AbsPos, RelDist};
/// 
/// // Subtract two absolute distances to get once relative
/// assert_eq!(AbsPos(2.0) - AbsPos(1.0), RelDist(1.0));
/// 
/// // Add relative distance to an absolute one
/// assert_eq!(AbsPos(2.0) + RelDist(1.0), AbsPos(3.0));
/// assert_eq!(AbsPos(2.0) - RelDist(1.0), AbsPos(1.0));
/// ```
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AbsPos(pub f32);
basic_unit!(AbsPos);

impl Sub<AbsPos> for AbsPos {
    type Output = RelDist;
    
    #[inline(always)]
    fn sub(self, rhs: AbsPos) -> Self::Output {
        RelDist(self.0 - rhs.0)
    }
}

impl Add<RelDist> for AbsPos {
    type Output = AbsPos;

    #[inline(always)]
    fn add(self, rhs: RelDist) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<AbsPos> for RelDist {
    type Output = RelDist;

    #[inline(always)]
    fn add(self, rhs: AbsPos) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<RelDist> for AbsPos {
    fn add_assign(&mut self, rhs: RelDist) {
        self.0 += rhs.0;
    }
}

impl Sub<RelDist> for AbsPos {
    type Output = AbsPos;

    #[inline]
    fn sub(self, rhs: RelDist) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign<RelDist> for AbsPos {
    fn sub_assign(&mut self, rhs: RelDist) {
        self.0 -= rhs.0;
    }
}

/// The rel_dist distance represents a relative distance traveled by the 
/// 
/// # Unit
/// 
/// - Can be either radians or millimeters
/// 
/// ```rust
/// use syunit::*;
/// 
/// assert_eq!(RelDist(2.0), RelDist(1.0) + RelDist(1.0));
/// assert_eq!(RelDist(5.0), RelDist(2.5) * 2.0);
/// assert_eq!(RelDist(2.0), AbsPos(4.0) - AbsPos(2.0));
/// ```
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelDist(pub f32);
basic_unit!(RelDist);
additive_unit!(RelDist);

/// Represents a change in distance over time
/// 
/// # Unit
/// 
/// - Can be either radians per second or millimeters per second
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Velocity(pub f32);
basic_unit!(Velocity);
additive_unit!(Velocity);
derive_units!(RelDist, Velocity, Time);

impl Velocity {
    /// Create a new `Velocity` (in rad/s) from a given number of rounds per minute (`rpm`)
    pub fn from_rpm(rpm : f32) -> Self {
        Self(rpm / 60.0 * 2.0 * PI)
    }

    /// Get the number of rounds per minute this
    pub fn into_rpm(self) -> f32 {
        (self / 2.0 / PI * 60.0).0
    }
}

impl Div<Velocity> for f32 {
    type Output = Time;

    #[inline(always)]
    fn div(self, rhs: Velocity) -> Self::Output {
        Time(self / rhs.0)
    }
}

/// Represents a change in distance over time
/// 
/// # Unit
/// 
/// - Hertz (1 / seconds)
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Frequency(pub f32);
basic_unit!(Frequency);
additive_unit!(Frequency);

impl Div<Frequency> for f32 {
    type Output = Time;

    #[inline(always)]
    fn div(self, rhs: Frequency) -> Self::Output {
        Time(self / rhs.0)
    }
}

/// Represents a change in velocity over time
/// 
/// # Unit
/// 
/// - Can be either radians per second^2 or millimeters per second^2
/// 
/// ```
/// use syunit::*;
/// 
/// assert_eq!(Velocity(5.0), Acceleration(2.5) * Time(2.0));
/// assert_eq!(Acceleration(2.5), Velocity(5.0) / Time(2.0));
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Acceleration(pub f32); 
basic_unit!(Acceleration);
additive_unit!(Acceleration);
derive_units!(Velocity, Acceleration, Time);
derive_units!(Force, Acceleration, Inertia);

/// Represents a change in acceleration over time
/// 
/// # Unit
/// 
/// - Can be either radians per second^3 or millimeters per second^3
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Jolt(pub f32); 
basic_unit!(Jolt);
additive_unit!(Jolt);
derive_units!(Acceleration, Jolt, Time);

/// Represents an inertia, slowing down movement processes
/// 
/// # Unit
/// 
/// - Can be either kilogramm or kilogramm times meter^2
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Inertia(pub f32);
basic_unit!(Inertia);
additive_unit!(Inertia);

/// Represents a force, slowing down movement processes, eventually even overloading the component
/// 
/// # Unit
/// 
/// - Can be either Newton or Newtonmeter
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Force(pub f32);
basic_unit!(Force);
additive_unit!(Force);