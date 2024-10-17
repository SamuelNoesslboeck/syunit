use super::*;

// Distances 
    /// Represents metric meters
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Meters(pub f32);
    basic_unit!(Meters, m);
    conversion_unit!(Meters, RelDist, 0.001);

    /// Represents metric meters
    #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Millimeters(pub f32);
    basic_unit!(Millimeters, mm);
    conversion_unit!(Millimeters, RelDist, 1.0);
    conversion_unit!(Millimeters, Meters, 0.001);
// 