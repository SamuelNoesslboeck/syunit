#![crate_name = "syunit"]
#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]

// Units
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

/// The gamma distance represents the actual distance traveled by the component
/// 
/// # Unit
/// 
/// - Can be either radians or millimeters
/// 
//// # Operations
/// ```rust
/// use syunit::{Gamma, Delta};
/// 
/// // Subtract two absolute distances to get once relative
/// assert_eq!(Gamma(2.0) - Gamma(1.0), Delta(1.0));
/// 
/// // Add relative distance to an absolute one
/// assert_eq!(Gamma(2.0) + Delta(1.0), Gamma(3.0));
/// assert_eq!(Gamma(2.0) - Delta(1.0), Gamma(1.0));
/// ```
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Gamma(pub f32);
basic_unit!(Gamma);

impl Gamma {
    /// Does a force conversion of gamma-distance (absolute distance of component) to a phi-distance 
    /// (absolute distance for mathematical calculations)
    pub fn force_to_phi(self) -> Phi {
        Phi(self.0)
    }
}

impl Sub<Gamma> for Gamma {
    type Output = Delta;
    
    #[inline(always)]
    fn sub(self, rhs: Gamma) -> Self::Output {
        Delta(self.0 - rhs.0)
    }
}

impl Add<Delta> for Gamma {
    type Output = Gamma;

    #[inline(always)]
    fn add(self, rhs: Delta) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Gamma> for Delta {
    type Output = Delta;

    #[inline(always)]
    fn add(self, rhs: Gamma) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<Delta> for Gamma {
    fn add_assign(&mut self, rhs: Delta) {
        self.0 += rhs.0;
    }
}

impl Sub<Delta> for Gamma {
    type Output = Gamma;

    #[inline]
    fn sub(self, rhs: Delta) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Gamma {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}


/// Helper functions to force convert an array of gammas to phis
#[inline]
pub fn force_phis_from_gammas<const N : usize>(gammas : [Gamma; N]) -> [Phi; N] {
    let mut phis = [Phi::ZERO; N];
    for i in 0 .. N {
        phis[i] = gammas[i].force_to_phi();
    }
    phis
}

/// Helper functions to foce convert an array of phis to gammas
#[inline]
pub fn force_gammas_from_phis<const N : usize>(phis : [Phi; N]) -> [Gamma; N] {
    let mut gammas = [Gamma::ZERO; N];
    for i in 0 .. N {
        gammas[i] = phis[i].force_to_gamma();
    }
    gammas
}

/// The phi distance represents the mathematical distance used for calculations
/// 
/// # Unit
/// 
/// - Can be either radians or millimeters
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Phi(pub f32);
basic_unit!(Phi);

impl Phi {
    /// Does a force conversion of this phi-distance (absolute distance for mathematical calculations) to a gamma-distance 
    /// (absolute distance for components)
    pub fn force_to_gamma(self) -> Gamma {
        Gamma(self.0)
    }
}

impl Sub<Phi> for Phi {
    type Output = Delta;

    fn sub(self, rhs: Phi) -> Self::Output {
        Delta(self.0 - rhs.0)
    }
}

impl Add<Delta> for Phi {
    type Output = Phi;

    #[inline(always)]
    fn add(self, rhs : Delta) -> Self::Output {
        Phi(self.0 + rhs.0)
    }
}

impl Add<Phi> for Delta {
    type Output = Phi;

    #[inline(always)]
    fn add(self, rhs: Phi) -> Self::Output {
        Phi(self.0 + rhs.0)
    }
}

impl AddAssign<Delta> for Phi {
    fn add_assign(&mut self, rhs: Delta) {
        self.0 += rhs.0;
    }
}

impl Sub<Delta> for Phi {
    type Output = Phi;

    #[inline(always)]
    fn sub(self, rhs: Delta) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign<Delta> for Phi {
    fn sub_assign(&mut self, rhs: Delta) {
        self.0 += rhs.0;
    }
}

/// The delta distance represents a relative distance traveled by the 
/// 
/// # Unit
/// 
/// - Can be either radians or millimeters
/// 
/// ```rust
/// use syunit::*;
/// 
/// assert_eq!(Delta(2.0), Delta(1.0) + Delta(1.0));
/// assert_eq!(Delta(5.0), Delta(2.5) * 2.0);
/// assert_eq!(Delta(2.0), Gamma(4.0) - Gamma(2.0));
/// ```
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Delta(pub f32);
basic_unit!(Delta);
additive_unit!(Delta);

impl Delta {
    /// Creates a new delta distance from a starting point `start` and an endpoint `end`
    #[inline(always)]
    pub fn diff(start : Gamma, end : Gamma) -> Self {
        end - start
    }
}

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
derive_units!(Delta, Velocity, Time);

impl Div<Velocity> for f32 {
    type Output = Time;

    #[inline(always)]
    fn div(self, rhs: Velocity) -> Self::Output {
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