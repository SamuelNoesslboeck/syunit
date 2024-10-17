use crate::RelDist;
use crate::metric::Millimeter;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; 

use crate as syunit;

// Distances
    /// Represents metric meters
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Inch(pub f32);
    syunit::basic_unit!(Inch, in);
    syunit::conversion_unit!(Inch, RelDist, 25.4);
    syunit::conversion_unit!(Inch, Millimeter, 25.4); 
// 