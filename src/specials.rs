use core::ops::{Div, Mul};
use core::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; 

/// 
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpeedFactor(f32);

impl SpeedFactor {
    /// Maximum speed
    pub const MAX : Self = Self(1.0);
    /// Half of the speed
    pub const HALF : Self = Self(0.5);
}

impl SpeedFactor {
    /// Create a new speed factor 
    pub const unsafe fn from_unchecked(value : f32) -> Self {
        Self(value)
    }
}

impl TryFrom<f32> for SpeedFactor {
    type Error = f32;

    fn try_from(value : f32) -> Result<Self, Self::Error> {
        if (value <= 1.0) & (value > 0.0) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

impl From<SpeedFactor> for f32 {
    fn from(value: SpeedFactor) -> Self {
        value.0
    }
}

/// Helper error for String-Parsing
pub enum SpeedFactorFromStrError {
    /// The string given is not convertable to a float
    BadString(<f32 as FromStr>::Err),
    /// The float is invalid for a `SpeedFactor`
    BadValue(f32)
}

impl FromStr for SpeedFactor {
    type Err = SpeedFactorFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SpeedFactor::try_from(f32::from_str(s).map_err(
            |err| SpeedFactorFromStrError::BadString(err), 
        )?).map_err(
            |err| SpeedFactorFromStrError::BadValue(err)
        )
    }
}

impl Default for SpeedFactor {
    fn default() -> Self {
        Self(1.0)
    }
}

impl Mul<SpeedFactor> for SpeedFactor {
    type Output = SpeedFactor;

    fn mul(self, rhs: SpeedFactor) -> Self::Output {
        SpeedFactor(self.0 * rhs.0)
    }
}

// Multiplication
    impl Mul<f32> for SpeedFactor {
        type Output = SpeedFactor;

        fn mul(self, rhs: f32) -> Self::Output {
            SpeedFactor::try_from(self.0 * rhs)
                .expect("Multiplication of speed-factor returned bad value")
        }
    }

    impl Mul<SpeedFactor> for f32 {
        type Output = SpeedFactor;

        fn mul(self, rhs: SpeedFactor) -> Self::Output {
            SpeedFactor::try_from(self * rhs.0)
                .expect("Multiplication of speed-factor returned bad value")
        }
    }
// 

// Division
    impl Div<f32> for SpeedFactor {
        type Output = SpeedFactor;

        fn div(self, rhs: f32) -> Self::Output {
            SpeedFactor::try_from(self.0 * rhs)
                .expect("Multiplication of speed-factor returned bad value")
        }
    }

    impl Div<SpeedFactor> for f32 {
        type Output = SpeedFactor;

        fn div(self, rhs: SpeedFactor) -> Self::Output {
            SpeedFactor::try_from(self * rhs.0)
                .expect("Multiplication of speed-factor returned bad value")
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

// impl Into<embedded_hal::Direction> for Direction {
//     fn into(self) -> embedded_hal::Direction {
//         match self {
//             Self::CCW => embedded_hal::Direction::Downcounting,
//             Self::CW => embedded_hal::Direction::Upcounting
//         }
//     }
// }

// impl From<embedded_hal::Direction> for Direction {
//     fn from(value: embedded_hal::Direction) -> Self {
//         match value {
//             embedded_hal::Direction::Downcounting => Self::CCW,
//             embedded_hal::Direction::Upcounting => Self::CW
//         }
//     }
// }