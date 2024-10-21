#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; 

// #####################
// #    SpeedFactor    #
// #####################
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

        pub const fn as_f32(self) -> f32 {
            self.0
        }
    }

    impl From<Factor> for f32 {
        fn from(value: Factor) -> Self {
            value.0
        }
    }

    impl core::ops::Mul<Factor> for Factor {
        type Output = Factor;

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
//


/// Direction of movement
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Direction {
    /// Counterclockwise (`false` / `0`)
    CCW,
    /// Clockwise (`true` / `1`)
    #[default]
    CW
}

impl Direction {
    /// Creates a new `Direction` value from a bool
    /// - `true` is `CW`
    /// - `false` is `CCW`
    #[inline]
    pub fn from_bool(b : bool) -> Self {
        if b { Direction::CW } else { Direction::CCW }
    }

    /// Converts the given direction into a bool value for logic signals
    /// - `CW` is `true`
    /// - `CCW` is `false` 
    #[inline]
    pub fn as_bool(self) -> bool {
        match self {
            Direction::CCW => false,
            Direction::CW => true
        }
    }

    /// Parses a new `Direction` value from a `u8`
    /// - `0` is `CCW`
    /// - Everything else is `CW` 
    #[inline]
    pub fn from_u8(u : u8) -> Self {
        if u == 0 { Direction::CCW } else { Direction::CW }
    } 

    /// Converts the given `Direction` into a `u8` value
    #[inline]
    pub fn as_u8(self) -> u8 {
        match self {
            Direction::CCW => 0,
            Direction::CW => 1
        }
    }
}

// Conversions
impl Into<bool> for Direction {
    fn into(self) -> bool {
        self.as_bool()
    }
}

impl From<bool> for Direction {
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl Into<u8> for Direction {
    fn into(self) -> u8 {
        self.as_u8()
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}