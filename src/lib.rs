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
    pub use metric::MetricMM;
// 

// Helper import for local macro definitions
use crate as syunit;

// ########################
// #    General traits    #
// ########################
    /// General marker trait for all units
    pub trait Unit : 
        From<f32> + Into<f32> +
        Copy + Clone + Debug + Display +
        FromStr + core::fmt::Debug + core::fmt::Display +
        Mul<f32, Output = Self> + Div<f32, Output = Self> + Neg<Output = Self>
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

        /// Creates a new value of this unit using a `f32` value
        fn new(v : f32) -> Self
        where 
            Self : Sized;
    }

    pub trait AdditiveUnit : Unit +
        Add<Self, Output = Self> + Sub<Self, Output = Self> +
        AddAssign<Self> + SubAssign<Self> { }

    pub trait DerivableUnit<R : Unit, V : Unit> : Unit +
        Div<V, Output = R> + Div<R, Output = V> { }

    pub trait IntegrableUnit<R : Unit, V : Unit> : Unit +
        Mul<V, Output = R> { }

    pub trait UnitSet {
        type Time : Unit + AdditiveUnit;

        type Position : 
            Unit +
            AddAssign<Self::Distance> + SubAssign<Self::Distance> +
            Add<Self::Distance, Output = Self::Position> + Sub<Self::Distance, Output = Self::Position>;

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
            type Output = Frequency;

            #[inline(always)]
            fn div(self, rhs: Seconds) -> Self::Output {
                Frequency(self / rhs.0)
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
        pub struct Frequency(pub f32);
        basic_unit!(Frequency);
        additive_unit!(Frequency);

        impl Div<Frequency> for f32 {
            type Output = Seconds;

            #[inline(always)]
            fn div(self, rhs: Frequency) -> Self::Output {
                Seconds(self / rhs.0)
            }
        }
    // 

    /*
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
*/