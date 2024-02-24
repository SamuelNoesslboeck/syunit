#![crate_name = "syunit"]
#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]

// Units
use core::ops::{Add, AddAssign, Div, Sub, SubAssign};
use core::time::Duration;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; 

// Helper import for local macro definitions
use crate as syunit;

/// General marker trait for all units
pub trait Unit : Into<f32> { 
    /// Creates a new value of this unit using a `f32` value
    fn new(v : f32) -> Self
    where 
        Self : Sized;
}

/// Helper macro that implements everything needed to do +,-,+=,-= operations with the unit itself
#[macro_export]
macro_rules! additive_unit {
    ( $unit:ident ) => {
        impl core::ops::Add<$unit> for $unit {
            type Output = $unit;
        
            #[inline(always)]
            fn add(self, rhs: $unit) -> Self::Output {
                $unit(self.0 + rhs.0)
            }
        }

        impl core::ops::AddAssign<$unit> for $unit {
            #[inline(always)]
            fn add_assign(&mut self, rhs: $unit) {
                self.0 += rhs.0;
            }
        }        
        
        impl core::ops::Sub<$unit> for $unit {
            type Output = $unit;
        
            #[inline(always)]
            fn sub(self, rhs: $unit) -> Self::Output {
                $unit(self.0 - rhs.0) 
            }
        }

        impl core::ops::SubAssign<$unit> for $unit {
            #[inline]
            fn sub_assign(&mut self, rhs : $unit) {
                self.0 -= rhs.0
            }
        }
    };
}

/// Implements the basics for a unit
#[macro_export]
macro_rules! basic_unit {
    ( $a:ident ) => {      
        impl $a {
            /// Zero value of this unit (0.0)
            pub const ZERO : Self = Self(0.0);
            /// Positive Infinity value of this unit (f32::INFINITY)
            pub const INFINITY : Self = Self(f32::INFINITY);
            /// Negative Infinity value of this unit (f32::INFINITY)
            pub const NEG_INFINITY : Self = Self(f32::NEG_INFINITY);
            /// NaN value of this unit (f32::NAN)
            pub const NAN : Self = Self(f32::NAN);

            /// Returns the absolute value of the unit 
            #[inline(always)]
            pub fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns `true` if this units value is neither NaN nor Infinite
            #[inline(always)]
            pub fn is_finite(self) -> bool {
                self.0.is_finite()
            }

            /// Returns `true` if this units value is neither NaN, Infinite or zero
            #[inline(always)]
            pub fn is_normal(self) -> bool {
                self.0.is_normal()
            }

            /// Returns `true` if this units value is Nan
            #[inline(always)]
            pub fn is_nan(self) -> bool {
                self.0.is_nan()
            }

            /// Returns the unit raised to the given integer power `pow`
            #[inline(always)]
            pub fn powi(self, pow : i32) -> Self {
                Self(self.0.powi(pow))
            }

            /// Returns the unit raised to the given power `pow`
            #[inline(always)]
            pub fn powf(self, pow : f32) -> Self {
                Self(self.0.powf(pow))
            }

            /// Returns the sin of this units value
            #[inline(always)]
            pub fn sin(self) -> f32 {
                self.0.sin()
            }

            /// Returns the cos of this units value
            #[inline(always)]
            pub fn cos(self) -> f32 {
                self.0.tan()
            }

            /// Returns the tan of this units value
            #[inline(always)]
            pub fn tan(self) -> f32 {
                self.0.tan()
            }

            /// Returns `true` if the sign bit of this value is negative (value smaller than 0.0, -0.0 included)
            pub fn is_sign_negative(self) -> bool { 
                self.0.is_sign_negative()
            }

            /// Returns `true` if the sign bit of this value is positive (value smaller than 0.0, -0.0 included)
            pub fn is_sign_positive(self) -> bool {
                self.0.is_sign_positive()
            }

            /// Returns the smaller value of this and another unit
            #[inline(always)]
            pub fn min(self, other : Self) -> Self {
                Self(self.0.min(other.0))
            }

            /// Return the bigger value of this and another unit
            #[inline(always)]
            pub fn max(self, other : Self) -> Self {
                Self(self.0.max(other.0))
            }
            
            /// Return the bigger value of this and another unit, working with references
            #[inline(always)]
            pub fn max_ref<'a>(&'a self, other : &'a Self) -> &'a Self {
                if *self < *other {
                    other
                } else {
                    self
                }
            }

            /// Return the bigger value of this and another unit, working with references
            #[inline(always)]
            pub fn min_ref<'a>(&'a self, other : &'a Self) -> &'a Self {
                if *self > *other {
                    other
                } else {
                    self
                }
            }
        }

        impl syunit::Unit for $a { 
            fn new(v : f32) -> Self {
                Self(v)
            }
        }

        impl core::str::FromStr for $a {
            type Err = <f32 as core::str::FromStr>::Err;
        
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(s.parse::<f32>()?))
            }
        }

        impl core::fmt::Display for $a {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl core::fmt::Debug for $a {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($a), self.0))
            }
        }

        impl core::convert::Into<f32> for $a {
            #[inline(always)]
            fn into(self) -> f32 {
                self.0
            }
        }

        // Ref
            
        // 

        // Negation
            impl core::ops::Neg for $a {
                type Output = Self;
                
                #[inline(always)]
                fn neg(self) -> Self::Output {
                    Self(-self.0)
                }
            }
        //

        // Multiplication
            impl core::ops::Mul<f32> for $a {
                type Output = $a;
                
                #[inline(always)]
                fn mul(self, rhs: f32) -> Self::Output {
                    $a(self.0 * rhs)
                }
            }

            impl core::ops::Mul<$a> for f32 {
                type Output = $a;

                #[inline(always)]
                fn mul(self, rhs : $a) -> Self::Output {
                    $a(self * rhs.0)
                }
            }
        // 
        
        // Division
            impl core::ops::Div<f32> for $a {
                type Output = $a;
            
                #[inline(always)]
                fn div(self, rhs: f32) -> Self::Output {
                    $a(self.0 / rhs)
                }
            }

            impl core::ops::Div<$a> for $a {
                type Output = f32;

                #[inline(always)]
                fn div(self, rhs : $a) -> Self::Output {
                    self.0 / rhs.0
                }
            }
        // 
    };
}

/// Implements everything required to form a "derive over time like"-connection between the given units
#[macro_export]
macro_rules! derive_units {
    ( $dist:ident, $vel:ident, $time:ident ) => {
        impl core::ops::Mul<$time> for $vel {
            type Output = $dist;
        
            #[inline(always)]
            fn mul(self, rhs: $time) -> Self::Output {
                $dist(self.0 * rhs.0)
            }
        }

        impl core::ops::Mul<$vel> for $time {
            type Output = $dist;
            
            #[inline(always)]
            fn mul(self, rhs: $vel) -> Self::Output {
                $dist(self.0 * rhs.0)
            }
        }

        impl core::ops::Div<$vel> for $dist {
            type Output = $time;
        
            #[inline(always)]
            fn div(self, rhs: $vel) -> Self::Output {
                $time(self.0 / rhs.0)
            }
        }

        impl core::ops::Div<$time> for $dist {
            type Output = $vel;
        
            #[inline(always)]
            fn div(self, rhs: $time) -> Self::Output {
                $vel(self.0 / rhs.0)
            }
        }
    };
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

/// Add two arrays of units
/// 
/// # Example
/// 
/// A simple example would be adding some delta distances to some gamma distances
/// 
/// ```rust
/// use syunit::*;
/// 
/// let gammas = [ Gamma(2.0), Gamma(1.0), Gamma(-3.5) ];
/// let deltas = [ Delta(1.2), Delta(3.5), Delta(0.5) ];
/// 
/// assert!(compare_unit_arrays(add_unit_arrays(gammas, deltas), [ Gamma(3.2), Gamma(4.5), Gamma(-3.0) ]));
/// ```
pub fn add_unit_arrays<U, Rhs, const C : usize>(base : [U; C], rhs : [Rhs; C]) -> [U::Output; C]
where
    U : Add<Rhs>,
    U : Copy,
    Rhs : Copy
{
    // Safe as zeroed, as it will not be read before being set
    let mut result : [U::Output; C] = unsafe { core::mem::zeroed() };

    for i in 0 .. C {
        result[i] = base[i] + rhs[i];
    }

    result
}

/// Subtract two arrays of units
/// 
/// # Example
/// 
/// A simple example would be subtracting some delta distances to some gamma distances
/// 
/// ```rust
/// use syunit::*;
/// 
/// let gammas = [ Gamma(2.2), Gamma(1.0), Gamma(-3.5) ];
/// let deltas = [ Delta(1.2), Delta(3.5), Delta(0.5) ];
/// 
/// assert!(compare_unit_arrays(sub_unit_arrays(gammas, deltas), [ Gamma(1.0), Gamma(-2.5), Gamma(-4.0) ]));
/// ```
pub fn sub_unit_arrays<U, Rhs, const C : usize>(base : [U; C], rhs : [Rhs; C]) -> [U::Output; C]
where
    U : Sub<Rhs>,
    U : Copy,
    Rhs : Copy
{
    // Safe as zeroed, as it will not be read before being set
    let mut result : [U::Output; C] = unsafe { core::mem::zeroed() };

    for i in 0 .. C {
        result[i] = base[i] - rhs[i];
    }

    result
}

/// Compare two unit arrays
/// 
/// # Example
/// 
/// ```rust
/// use syunit::*;
/// 
/// let gammas_ori = [ Gamma(2.0), Gamma(1.0), Gamma(-3.5) ];
/// let gammas_same = [ Gamma(2.0), Gamma(1.0), Gamma(-3.5) ];
/// let gammas_diff = [ Gamma(1.0), Gamma(2.0), Gamma(3.5) ];
/// 
/// assert!(compare_unit_arrays(gammas_ori, gammas_same));
/// assert!(!compare_unit_arrays(gammas_ori, gammas_diff)); 
/// ```
pub fn compare_unit_arrays<U, const C : usize>(base : [U; C], rhs : [U; C]) -> bool
where
    U : PartialEq,
    U : Copy
{
    for i in 0 .. C {
        if base[i] != rhs[i] {
            return false;
        }
    }

    true
}