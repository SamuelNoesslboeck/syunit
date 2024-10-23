use crate::metric::Millimeters;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; 

use crate as syunit;

// Distances
    /// Represents metric meters
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Inches(pub f32);
    syunit::basic_unit!(Inches, "in");
    syunit::impl_conversion!(Inches, Millimeters, 25.4); 
// 