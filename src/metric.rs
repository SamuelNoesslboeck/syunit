use crate::RelDist;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; 

use crate as syunit;

// Distances 
    /// Represents metric meters
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Meter(pub f32);
    syunit::basic_unit!(Meter, m);
    syunit::conversion_unit!(Meter, RelDist, 0.001);

    /// Represents metric meters
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Millimeter(pub f32);
    syunit::basic_unit!(Millimeter, mm);
    syunit::conversion_unit!(Millimeter, RelDist, 1.0);
    syunit::conversion_unit!(Millimeter, Meter, 0.001);
// 