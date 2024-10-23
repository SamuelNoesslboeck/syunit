#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; 

use crate::{Seconds, UnitSet, PositionRad, Radians, RadPerSecond, RadPerSecond2, RadPerSecond3};

use crate as syunit;

// ###############################
// #    Metric Millimeter Set    #
// ###############################
    // Units
        /// Represents a position in metric millimeters (mm)
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct PositionMM(pub f32);
        syunit::basic_unit!(PositionMM, "mm");
        syunit::position_unit!(PositionMM, Millimeters);
        syunit::impl_full_conversion!(PositionRad, Millimeters, PositionMM);

        /// Represents metric millimeters (mm)
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct Millimeters(pub f32);
        syunit::basic_unit!(Millimeters, "mm");
        syunit::additive_unit!(Millimeters);
        syunit::derive_units!(Millimeters, MMPerSecond, Seconds);
        syunit::impl_mul_bidir!(Radians, Millimeters, Millimeters);

        /// Represents metric millimeters per second (mm/s)
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct MMPerSecond(pub f32);
        syunit::basic_unit!(MMPerSecond, "mm/s");
        syunit::additive_unit!(MMPerSecond);
        syunit::derive_units!(MMPerSecond, MMPerSecond2, Seconds);
        syunit::impl_full_conversion!(RadPerSecond, Millimeters, MMPerSecond);

        /// Represents metric millimeters per second squared (mm/s^2)
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct MMPerSecond2(pub f32);
        syunit::basic_unit!(MMPerSecond2, "mm/s^2");
        syunit::additive_unit!(MMPerSecond2);
        syunit::derive_units!(MMPerSecond2, MMPerSecond3, Seconds);
        syunit::impl_full_conversion!(RadPerSecond2, Millimeters, MMPerSecond2);

        /// Represents metric meters
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct MMPerSecond3(pub f32);
        syunit::basic_unit!(MMPerSecond3, "mm/s^3");
        syunit::additive_unit!(MMPerSecond3);
        syunit::impl_full_conversion!(RadPerSecond3, Millimeters, MMPerSecond3);

        /// Represents Newtons
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct Newtons(pub f32);
        syunit::basic_unit!(Newtons, "N");
        syunit::additive_unit!(Newtons);
        syunit::impl_full_conversion!(MMPerSecond2, Kilogramms, Newtons, 0.001); 

        /// Represents metric Kilogramms
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct Kilogramms(pub f32);
        syunit::basic_unit!(Kilogramms, "kg");
        syunit::additive_unit!(Kilogramms);
        syunit::inertia_unit!(Kilogramms, f32, Kilogramms);
        syunit::inertia_unit!(Kilogramms, Millimeters, KgMeter2, 0.000_001);
    // 

    /// A [UnitSet] centered around metric [Millimeters]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct MetricMM { }

    impl UnitSet for MetricMM {
        type Time = Seconds;

        type Position = PositionMM;

        type Distance = Millimeters;
        type Velocity = MMPerSecond;
        type Acceleration = MMPerSecond2;
        type Jolt = MMPerSecond3;

        type Force = Newtons;
        type Inertia = Kilogramms;
    }
// 

// #############################
// #    Metric Rotation Set    #
// #############################
    // Units
        /// Represents metric Newtonmeters
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct NewtonMeters(pub f32);
        syunit::basic_unit!(NewtonMeters, "Nm");
        syunit::additive_unit!(NewtonMeters);
        syunit::impl_full_conversion!(KgMeter2, RadPerSecond2, NewtonMeters);   
        syunit::impl_full_conversion!(Newtons, Millimeters, NewtonMeters, 0.001); 

        /// Represents a second moment of inertia in Kilogramms times meters squared
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct KgMeter2(pub f32);
        syunit::basic_unit!(KgMeter2, "kgm^2");
        syunit::additive_unit!(KgMeter2);
        syunit::inertia_unit!(KgMeter2, f32, KgMeter2);
        syunit::inertia_unit!(KgMeter2, Millimeters, Kilogramms, 1_000_000.0);
    // 

    /// A [UnitSet] expressing rotary units centered around [Radians]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Rotary { }

    impl UnitSet for Rotary {
        type Time = Seconds;

        type Position = PositionRad;

        type Distance = Radians;
        type Velocity = RadPerSecond;
        type Acceleration = RadPerSecond2;
        type Jolt = RadPerSecond3;

        type Force = NewtonMeters;
        type Inertia = KgMeter2;
    }
// 


// ##########################
// #    Metric Meter Set    #
// ##########################
    // Units
        /// Represents a position in metric 
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct PositionM(pub f32);
        syunit::basic_unit!(PositionM, "m");
        syunit::position_unit!(PositionM, Meters);
        syunit::impl_full_conversion!(PositionRad, Meters, PositionM);
        syunit::impl_conversion!(Meters, Millimeters, 1000.0);

        /// Represents metric meters (m)
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct Meters(pub f32);
        syunit::basic_unit!(Meters, "m");
        syunit::additive_unit!(Meters);
        // syunit::derive_units!(Meters, MMPerSecond, Seconds);
        syunit::impl_mul_bidir!(Radians, Meters, Meters);
    //

    // TODO: Finish metric units set
//