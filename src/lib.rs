#![crate_name = "syunit"]
#![doc = include_str!("../README.md")]
#![no_std]
// Rules
#![deny(missing_docs)]

use core::fmt::{Debug, Display};
use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use core::str::FromStr;
use core::time::Duration;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; 

// ####################
// #    Submodules    #
// ####################
    /// Functions for handling units
    mod funcs;
    pub use funcs::*;

    /// Macros for creating units and fast implementations between them
    pub mod macros;

    // Special units
    mod specials;
    pub use specials::*;

    // Unit systems
    /// Imperial units of measurement
    pub mod imperial;

    /// Metric units of measurement and useful [UnitSets](UnitSet)
    pub mod metric;
    pub use metric::{MetricMM, Rotary};

    /// Lazy import of the library
    /// 
    /// Imports both the root and [metric] module
    pub mod prelude {
        pub use crate::*;
        pub use crate::metric::*;
        // Using metric for examples because imperial sucks
    }
// 

// Helper import for local macro definitions
use crate as syunit;

// ########################
// #    General traits    #
// ########################
    /// General trait for all units, defines the basic constraints required to work with units
    pub trait Unit : 
        From<f32> + Into<f32> +
        Copy + Clone + Debug + Display + PartialEq + PartialOrd + Default + 
        FromStr + core::fmt::Debug + core::fmt::Display +
        Mul<f32, Output = Self> + Div<f32, Output = Self> + Div<Self, Output = f32> + Neg<Output = Self> +
        Mul<Factor, Output = Self>
    where  
        Self : Sized
    { 
        /// Zero value of this unit (0.0)
        const ZERO : Self;
        /// Positive Infinity value of this unit (f32::INFINITY)
        const INFINITY : Self;
        /// Negative Infinity value of this unit (f32::INFINITY)
        const NEG_INFINITY : Self;

        /// NaN value of this unit (f32::NAN)
        /// 
        /// ```rust
        /// use syunit::prelude::*;
        /// 
        /// assert!((Radians::ZERO / 0.0).is_nan());                // Result is Radians::NAN 
        /// assert!((Radians::ZERO / Radians::ZERO).is_nan());      // Result is f32::NAN
        /// ```
        const NAN : Self;

        /// Returns the absolute value of the unit 
        #[inline(always)]
        fn abs(self) -> Self {
            Self::from(self.into().abs())
        }

        /// Returns `true` if this units value is neither [NaN](Unit::NAN) nor [Infinite](Unit::INFINITY)
        #[inline(always)]
        fn is_finite(self) -> bool {
            self.into().is_finite()
        }

        /// Returns `true` if this units value is neither [NaN](Unit::NAN), [Infinite](Unit::INFINITY) or zero
        #[inline(always)]
        fn is_normal(self) -> bool {
            self.into().is_normal()
        }

        /// Returns `true` if this units value is [NaN](Unit::NAN)
        #[inline(always)]
        fn is_nan(self) -> bool {
            self.into().is_nan()
        }

        /// Returns the unit raised to the given integer power `pow`
        #[inline(always)]
        fn powi(self, pow : i32) -> Self {
            Self::from(self.into().powi(pow))
        }

        /// Returns the unit raised to the given power `pow`
        #[inline(always)]
        fn powf(self, pow : f32) -> Self {
            Self::from(self.into().powf(pow))
        }

        /// Returns the sin of this units value
        #[inline(always)]
        fn sin(self) -> f32 {
            self.into().sin()
        }

        /// Returns the cos of this units value
        #[inline(always)]
        fn cos(self) -> f32 {
            self.into().tan()
        }

        /// Returns the tan of this units value
        #[inline(always)]
        fn tan(self) -> f32 {
            self.into().tan()
        }

        /// Get the direction of the value (positive or negative)
        /// 
        /// `0.0` will be accounted as positive
        fn get_direction(self) -> syunit::Direction {
            if self >= Self::ZERO {
                syunit::Direction::CW
            } else {
                syunit::Direction::CCW
            }
        }

        /// Returns `true` if the sign bit of this value is negative (value smaller than `0.0`, `-0.0` included)
        fn is_sign_negative(self) -> bool { 
            self.into().is_sign_negative()
        }

        /// Returns `true` if the sign bit of this value is positive (value smaller than `0.0`, `-0.0` included)
        fn is_sign_positive(self) -> bool {
            self.into().is_sign_positive()
        }

        // Comparision
            /// Compare this unit with another instance and return the bigger one
            #[inline(always)]
            fn max(self, other : Self) -> Self {
                Self::from(self.into().max(other.into()))
            }

            /// Compare this unit with another instance and return the smaller one
            #[inline(always)]
            fn min(self, other : Self) -> Self {
                Self::from(self.into().min(other.into()))
            }

            /// Like [Unit::max], but with references
            #[inline(always)]
            fn max_ref<'a>(&'a self, other : &'a Self) -> &'a Self {
                if *self < *other {
                    other
                } else {
                    self
                }
            }

            /// Like [Unit::min], but with references
            #[inline(always)]
            fn min_ref<'a>(&'a self, other : &'a Self) -> &'a Self {
                if *self > *other {
                    other
                } else {
                    self
                }
            }
        //
    }

    /// Marker traits for units, that can be added and subtracted from themselfs
    pub trait AdditiveUnit : Unit +
        Add<Self, Output = Self> + Sub<Self, Output = Self> +
        AddAssign<Self> + SubAssign<Self> { }
    
    /// Marker trait for units that can be derived by a variable `V` to form the result `Result`
    /// 
    /// ```text
    /// dSelf / dV = Result
    /// ```
    /// 
    /// The best example would be implementing this trait for a distance unit, with the variable being a time unit, then the result would be a velocity unit
    pub trait DerivableUnit<V : Unit> : Unit + Div<V, Output = Self::Result> + Div<Self::Result, Output = V> { 
        /// The result of the derivative
        type Result : Unit;
    }

    /// Marker trait for units that can be integrated by a variable `V` to form the result `Result`
    /// 
    /// ```text
    /// Integral( Self * dV ) = Result
    /// ```
    /// 
    /// The best example would be implementing this trait for a distance unit, with the variable being a time unit, then the result would be a velocity unit
    pub trait IntegrableUnit<V : Unit> : Unit + Mul<V, Output = Self::Result> { 
        /// The result of the integral
        type Result : Unit;
    }

    /// A set of units that have a strong relationship to each other
    /// 
    /// This trait is useful when defining a function/struct/trait while not wanting to specify any units
    /// 
    /// ```rust
    /// use syunit::UnitSet;
    /// 
    /// fn get_distance<U : UnitSet>(vel : U::Velocity, time : U::Time) -> U::Distance {
    ///     vel * time      // Compiler automatically checks if the types match
    /// }
    /// ```
    pub trait UnitSet : Copy + Clone + Debug + Default {
        /// Time unit of the [UnitSet], most likely [Seconds]
        type Time : Unit + AdditiveUnit;

        /// Position unit of the [UnitSet]
        /// 
        /// Uses the same physical unit as [UnitSet::Distance], however it represents an *absolute position*, not a distance! 
        /// It does not implement [AdditiveUnit], but [UnitSet::Distance] can be added an subtracted from it. For more information on Position and Distance units see the [main page](syunit)
        /// 
        /// ```rust
        /// use syunit::prelude::*;
        /// 
        /// fn get_difference<U : UnitSet>(pos_start : U::Position, pos_end : U::Position) -> U::Distance {
        ///     pos_end - pos_start     // Compiler automatically checks if the types match
        /// }
        /// ```
        type Position : 
            Unit +
            AddAssign<Self::Distance> + SubAssign<Self::Distance> +
            Add<Self::Distance, Output = Self::Position> + Sub<Self::Distance, Output = Self::Position> + Sub<Self::Position, Output = Self::Distance>;

        // Kinematics
            /// Distance unit of the [UnitSet]
            /// 
            /// Unlike [UnitSet::Position], this unit represents a *relative distance*, not a position! See [UnitSet::Position] for more infos!
            type Distance : 
                Unit + AdditiveUnit +
                DerivableUnit<Self::Time, Result = Self::Velocity>;

            /// Velocity unit of the [UnitSet]
            type Velocity : 
                Unit + AdditiveUnit +
                DerivableUnit<Self::Time, Result = Self::Acceleration> +
                IntegrableUnit<Self::Time, Result = Self::Distance>;

            /// Acceleration unit of the [UnitSet]
            type Acceleration :
                Unit + AdditiveUnit +
                DerivableUnit<Self::Time, Result = Self::Jolt> +
                IntegrableUnit<Self::Time, Result = Self::Velocity>;

            /// Jolt unit of the [UnitSet]
            type Jolt :
                Unit + AdditiveUnit +
                IntegrableUnit<Self::Time, Result = Self::Acceleration>;
        // 

        // Dynamics
            /// Force unit of the [UnitSet], required for more advanced calculations
            type Force : 
                Unit + AdditiveUnit +
                Div<Self::Inertia, Output = Self::Acceleration> +
                Div<Self::Acceleration, Output = Self::Inertia>;

            /// Inertia unit of the [UnitSet], required for more advanced calculations
            type Inertia :
                Unit + AdditiveUnit +
                Mul<Self::Acceleration, Output = Self::Force>;
        // 
    }

    /// A helper trait for calculations with inertia units
    pub trait InertiaUnit<B : Clone + Copy + Into<f32>> : Unit {
        /// The inertia type that will be created when reducing the inertia
        type Reduced : Unit;

        // TODO: Add mathematical documentation
        /// Reduce the inertia
        fn reduce(self, ratio : B) -> Self::Reduced {
            Self::Reduced::from(self.into() * ratio.into() * ratio.into())
        }

        // TODO: Add mathematical documentation
        /// Reversal of [InertiaUnit::reduce]
        fn extend(reduced : Self::Reduced, ratio : B) -> Self {
            Self::from(reduced.into() / ratio.into() / ratio.into())
        }
    }
// 

// #######################
// #    General Units    #
// #######################
    // Time
        /// Represents a time in seconds as a [f32]
        /// 
        /// ```rust
        /// use core::time::Duration;
        /// 
        /// use syunit::prelude::*;
        /// 
        /// // Duration conversion
        /// assert_eq!(Seconds(2.0), Duration::from_secs(2).into());
        /// assert_eq!(Seconds(0.005), Duration::from_millis(5).into());
        /// 
        /// // Basic maths
        /// assert_eq!(Seconds(3.0) + Seconds(2.0), Seconds(5.0));
        /// assert_eq!(Seconds(3.0) - Seconds(2.0), Seconds(1.0)); 
        /// assert_eq!(Seconds(3.0) * 2.0, Seconds(6.0));
        /// assert_eq!(Seconds(3.0) / -2.0, Seconds(-1.5));
        /// 
        /// // Comparisions
        /// assert!(Seconds(1.0) > Seconds(-1.0));
        /// ```
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct Seconds(pub f32);
        basic_unit!(Seconds);
        additive_unit!(Seconds);
        
        impl From<Seconds> for Duration {
            #[inline(always)]
            fn from(value : Seconds) -> Self {
                Duration::from_secs_f32(value.0)
            }
        }

        impl From<Duration> for Seconds {
            #[inline(always)]
            fn from(value : Duration) -> Self {
                Self(value.as_secs_f32())
            }
        }
    //

    // Frequency
        /// Represents a freqency in Hertz (or 1 / Seconds)
        /// 
        /// ```rust
        /// use syunit::prelude::*;
        /// 
        /// // Seconds conversion
        /// assert_eq!(Hertz(4.0), 1.0 / Seconds(0.25));
        /// assert_eq!(1.0 / Hertz(5.0), Seconds(0.2));
        /// 
        /// // Basic maths
        /// assert_eq!(Hertz(3.0) + Hertz(2.0), Hertz(5.0));
        /// assert_eq!(Hertz(3.0) - Hertz(2.0), Hertz(1.0)); 
        /// assert_eq!(Hertz(3.0) * 2.0, Hertz(6.0));
        /// assert_eq!(Hertz(3.0) / -2.0, Hertz(-1.5));
        /// ```
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct Hertz(pub f32);
        basic_unit!(Hertz, "Hz");
        additive_unit!(Hertz);

        impl Mul<Hertz> for Seconds {
            type Output = f32;

            #[inline]
            fn mul(self, rhs: Hertz) -> Self::Output {
                self.0 * rhs.0
            }
        }

        impl Mul<Seconds> for Hertz {
            type Output = f32;

            #[inline]
            fn mul(self, rhs: Seconds) -> Self::Output {
                self.0 * rhs.0
            }
        }

        impl Div<Hertz> for f32 {
            type Output = Seconds;

            #[inline(always)]
            fn div(self, rhs: Hertz) -> Self::Output {
                Seconds(self / rhs.0)
            }
        }

        impl Div<Seconds> for f32 {
            type Output = Hertz;

            #[inline(always)]
            fn div(self, rhs: Seconds) -> Self::Output {
                Hertz(self / rhs.0)
            }
        }
    // 

    /// Represents a position in Radians
    /// 
    /// ```
    /// use syunit::prelude::*;
    /// 
    /// // Position math
    /// assert_eq!(PositionRad(3.0) + Radians(2.0), PositionRad(5.0));
    /// assert_eq!(PositionRad(3.0) - PositionRad(2.0), Radians(1.0)); 
    /// 
    /// // Basic maths
    /// assert_eq!(Radians(3.0) * 2.0, Radians(6.0));
    /// assert_eq!(Radians(3.0) / -2.0, Radians(-1.5));
    /// ```
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct PositionRad(pub f32);
    syunit::basic_unit!(PositionRad, "rad");
    syunit::position_unit!(PositionRad, Radians);

    /// Represents Radians (rad)
    /// 
    /// ```rust
    /// use syunit::prelude::*;
    /// 
    /// // Basic maths
    /// assert_eq!(Radians(3.0) + Radians(2.0), Radians(5.0));
    /// assert_eq!(Radians(3.0) - Radians(2.0), Radians(1.0)); 
    /// assert_eq!(Radians(3.0) * 2.0, Radians(6.0));
    /// assert_eq!(Radians(3.0) / -2.0, Radians(-1.5));
    /// 
    /// // Angle calculations
    /// assert_eq!(Radians(2.0) * Millimeters(3.0), Millimeters(6.0));
    /// ```
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Radians(pub f32);
    syunit::basic_unit!(Radians, "rad");
    syunit::additive_unit!(Radians);
    syunit::derive_units!(Radians, RadPerSecond, Seconds);

    /// Represents Radians per second (rad/s)
    /// 
    /// ```rust
    /// use syunit::prelude::*;
    /// 
    /// // Basic maths
    /// assert_eq!(RadPerSecond(3.0) + RadPerSecond(2.0), RadPerSecond(5.0));
    /// assert_eq!(RadPerSecond(3.0) - RadPerSecond(2.0), RadPerSecond(1.0)); 
    /// assert_eq!(RadPerSecond(3.0) * 2.0, RadPerSecond(6.0));
    /// assert_eq!(RadPerSecond(3.0) / -2.0, RadPerSecond(-1.5));
    /// 
    /// // Angle calculations
    /// assert_eq!(RadPerSecond(2.0) * Millimeters(3.0), MMPerSecond(6.0));     // ang-velocity * radius = velocity
    /// ```
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct RadPerSecond(pub f32);
    syunit::basic_unit!(RadPerSecond, "rad/s");
    syunit::additive_unit!(RadPerSecond);
    syunit::derive_units!(RadPerSecond, RadPerSecond2, Seconds);

    /// Represents Radians per second squared (rad/s^2)
    /// 
    /// ```rust
    /// use syunit::prelude::*;
    /// 
    /// // Basic maths
    /// assert_eq!(RadPerSecond2(3.0) + RadPerSecond2(2.0), RadPerSecond2(5.0));
    /// assert_eq!(RadPerSecond2(3.0) - RadPerSecond2(2.0), RadPerSecond2(1.0)); 
    /// assert_eq!(RadPerSecond2(3.0) * 2.0, RadPerSecond2(6.0));
    /// assert_eq!(RadPerSecond2(3.0) / -2.0, RadPerSecond2(-1.5));
    /// 
    /// // Angle calculations
    /// assert_eq!(RadPerSecond2(2.0) * Millimeters(3.0), MMPerSecond2(6.0));     // ang-acceleration * radius = acceleration
    /// ```
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct RadPerSecond2(pub f32);
    syunit::basic_unit!(RadPerSecond2, "rad/s^2");
    syunit::additive_unit!(RadPerSecond2);
    syunit::derive_units!(RadPerSecond2, RadPerSecond3, Seconds);

    /// Represents Radians per second qubed (rad/s^3)
    /// 
    /// ```rust
    /// use syunit::prelude::*;
    /// 
    /// // Basic maths
    /// assert_eq!(RadPerSecond3(3.0) + RadPerSecond3(2.0), RadPerSecond3(5.0));
    /// assert_eq!(RadPerSecond3(3.0) - RadPerSecond3(2.0), RadPerSecond3(1.0)); 
    /// assert_eq!(RadPerSecond3(3.0) * 2.0, RadPerSecond3(6.0));
    /// assert_eq!(RadPerSecond3(3.0) / -2.0, RadPerSecond3(-1.5));
    /// 
    /// // Angle calculations
    /// assert_eq!(RadPerSecond3(2.0) * Millimeters(3.0), MMPerSecond3(6.0));     // ang-jolt * radius = jolt
    /// ```
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct RadPerSecond3(pub f32);
    syunit::basic_unit!(RadPerSecond3, "rad/s^3");
    syunit::additive_unit!(RadPerSecond3);
//