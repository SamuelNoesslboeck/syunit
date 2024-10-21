#![crate_name = "syunit"]
#![doc = include_str!("../README.md")]
#![no_std]
// #![deny(missing_docs)]

use core::fmt::{Debug, Display};
use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use core::str::FromStr;
use core::time::Duration;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; 

// ####################
// #    Submodules    #
// ####################
    mod funcs;
    pub use funcs::*;

    /// Macros for creating units
    pub mod macros;

    mod specials;
    pub use specials::*;

    // Unit systems
    /// Imperial units of measurement
    pub mod imperial;

    /// Metric units of measurement 
    pub mod metric;
    pub use metric::{MetricMM, Rotary};
// 

// Helper import for local macro definitions
use crate as syunit;

// ########################
// #    General traits    #
// ########################
    /// General marker trait for all units
    pub trait Unit : 
        From<f32> + Into<f32> +
        Copy + Clone + Debug + Display + PartialEq + PartialOrd + Default + 
        FromStr + core::fmt::Debug + core::fmt::Display +
        Mul<f32, Output = Self> + Div<f32, Output = Self> + Div<Self, Output = f32> + Neg<Output = Self>
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
        const NAN : Self;

        /// Returns the absolute value of the unit 
        #[inline(always)]
        fn abs(self) -> Self {
            Self::from(self.into().abs())
        }

        /// Returns `true` if this units value is neither NaN nor Infinite
        #[inline(always)]
        fn is_finite(self) -> bool {
            self.into().is_finite()
        }

        /// Returns `true` if this units value is neither NaN, Infinite or zero
        #[inline(always)]
        fn is_normal(self) -> bool {
            self.into().is_normal()
        }

        /// Returns `true` if this units value is Nan
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

        /// Returns `true` if the sign bit of this value is negative (value smaller than 0.0, -0.0 included)
        fn is_sign_negative(self) -> bool { 
            self.into().is_sign_negative()
        }

        /// Returns `true` if the sign bit of this value is positive (value smaller than 0.0, -0.0 included)
        fn is_sign_positive(self) -> bool {
            self.into().is_sign_positive()
        }

        // Comparision
            /// Return the bigger value of this and another unit
            #[inline(always)]
            fn max(self, other : Self) -> Self {
                Self::from(self.into().max(other.into()))
            }

            /// Returns the smaller value of this and another unit
            #[inline(always)]
            fn min(self, other : Self) -> Self {
                Self::from(self.into().min(other.into()))
            }

            /// Return the bigger value of this and another unit, working with references
            #[inline(always)]
            fn max_ref<'a>(&'a self, other : &'a Self) -> &'a Self {
                if *self < *other {
                    other
                } else {
                    self
                }
            }

            /// Return the bigger value of this and another unit, working with references
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

    pub trait AdditiveUnit : Unit +
        Add<Self, Output = Self> + Sub<Self, Output = Self> +
        AddAssign<Self> + SubAssign<Self> { }

    pub trait DerivableUnit<R : Unit, V : Unit> : Unit +
        Div<V, Output = R> + Div<R, Output = V> { }

    pub trait IntegrableUnit<R : Unit, V : Unit> : Unit +
        Mul<V, Output = R> { }

    pub trait UnitSet : Copy + Clone + Debug + Default {
        type Time : Unit + AdditiveUnit;

        type Position : 
            Unit +
            AddAssign<Self::Distance> + SubAssign<Self::Distance> +
            Add<Self::Distance, Output = Self::Position> + Sub<Self::Distance, Output = Self::Position> + Sub<Self::Position, Output = Self::Distance>;

        // Kinematics
            type Distance : 
                Unit + AdditiveUnit +
                DerivableUnit<Self::Velocity, Self::Time>;

            type Velocity : 
                Unit + AdditiveUnit +
                DerivableUnit<Self::Acceleration, Self::Time> +
                IntegrableUnit<Self::Distance, Self::Time>;

            type Acceleration :
                Unit + AdditiveUnit +
                DerivableUnit<Self::Jolt, Self::Time> +
                IntegrableUnit<Self::Velocity, Self::Time>;

            type Jolt :
                Unit + AdditiveUnit +
                IntegrableUnit<Self::Acceleration, Self::Time>;
        // 

        // Dynamics
            type Force : 
                Unit + AdditiveUnit +
                Div<Self::Inertia, Output = Self::Acceleration> +
                Div<Self::Acceleration, Output = Self::Inertia>;

            type Inertia :
                Unit + AdditiveUnit +
                Mul<Self::Acceleration, Output = Self::Force>;
        // 
    }

    pub trait TransformableSet<O : UnitSet, R : From<f32> + Into<f32>> : UnitSet 
    where
        // Conversion operations required for input
        Self::Position : Div<R, Output = O::Position>,
        Self::Distance : Div<R, Output = O::Distance>,
        Self::Velocity : Div<R, Output = O::Velocity>,
        Self::Acceleration : Div<R, Output = O::Acceleration>,
        Self::Jolt : Div<R, Output = O::Jolt>,
        Self::Force : Mul<R, Output = O::Force>,
        // Inertia currently not possible, as the ratio is doubled for the calculation of that, which messes up with units, maybe Area and volume are added in the future ...

        // Conversion operations required for output
        O::Position : Mul<R, Output = Self::Position>,
        O::Distance : Mul<R, Output = Self::Distance>,
        O::Velocity : Mul<R, Output = Self::Velocity>,
        O::Acceleration : Mul<R, Output = Self::Acceleration>,
        O::Jolt : Mul<R, Output = Self::Jolt>,
        O::Force : Div<R, Output = Self::Force>,
        // Inertia currently not possible, as the ratio is doubled for the calculation of that, which messes up with units, maybe Area and volume are added in the future ...
    {

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
        /// use syunit::*;
        /// 
        /// // Duration conversion
        /// assert_eq!(Time(2.0), Duration::from_secs(2).into());
        /// assert_eq!(Time(0.005), Duration::from_millis(5).into());
        /// 
        /// // Comparisions
        /// assert!(Time(1.0) > Time(-1.0));
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

        impl Div<Seconds> for f32 {
            type Output = Hertz;

            #[inline(always)]
            fn div(self, rhs: Seconds) -> Self::Output {
                Hertz(self / rhs.0)
            }
        }

        impl Mul<Factor> for Seconds {
            type Output = Seconds;

            #[inline(always)]
            fn mul(self, rhs: Factor) -> Self::Output {
                Self(self.0 * rhs.as_f32())
            }
        }
    //

    // Frequency
        /// Represents a change in distance over time
        /// 
        /// # Unit
        /// 
        /// - Hertz (1 / seconds)
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct Hertz(pub f32);
        basic_unit!(Hertz, "Hz");
        additive_unit!(Hertz);

        impl Div<Hertz> for f32 {
            type Output = Seconds;

            #[inline(always)]
            fn div(self, rhs: Hertz) -> Self::Output {
                Seconds(self / rhs.0)
            }
        }
    // 

    /// Represents a position in radians
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct PositionRad(pub f32);
    syunit::basic_unit!(PositionRad, "rad");
    syunit::position_unit!(PositionRad, Radians);

    /// Represents Radians (rad)
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Radians(pub f32);
    syunit::basic_unit!(Radians, "rad");
    syunit::additive_unit!(Radians);
    syunit::derive_units!(Radians, RadPerSecond, Seconds);

    /// Represents Radians per second (rad/s)
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct RadPerSecond(pub f32);
    syunit::basic_unit!(RadPerSecond, "rad/s");
    syunit::additive_unit!(RadPerSecond);
    syunit::derive_units!(RadPerSecond, RadPerSecond2, Seconds);

    /// Represents metric millimeters per second squared (mm/s^2)
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct RadPerSecond2(pub f32);
    syunit::basic_unit!(RadPerSecond2, "rad/s^2");
    syunit::additive_unit!(RadPerSecond2);
    syunit::derive_units!(RadPerSecond2, RadPerSecond3, Seconds);

    /// Represents metric meters
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct RadPerSecond3(pub f32);
    syunit::basic_unit!(RadPerSecond3, "rad/s^3");
    syunit::additive_unit!(RadPerSecond3);
//