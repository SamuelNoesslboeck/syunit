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

        /// Represents metric millimeters (mm)
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct Millimeters(pub f32);
        syunit::basic_unit!(Millimeters, "mm");
        syunit::additive_unit!(Millimeters);
        syunit::derive_units!(Millimeters, MMPerSecond, Seconds);

        /// Represents metric millimeters per second (mm/s)
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct MMPerSecond(pub f32);
        syunit::basic_unit!(MMPerSecond, "mm/s");
        syunit::additive_unit!(MMPerSecond);
        syunit::derive_units!(MMPerSecond, MMPerSecond2, Seconds);

        /// Represents metric millimeters per second squared (mm/s^2)
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct MMPerSecond2(pub f32);
        syunit::basic_unit!(MMPerSecond2, "mm/s^2");
        syunit::additive_unit!(MMPerSecond2);
        syunit::derive_units!(MMPerSecond2, MMPerSecond3, Seconds);

        /// Represents metric meters
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct MMPerSecond3(pub f32);
        syunit::basic_unit!(MMPerSecond3, "mm/s^3");
        syunit::additive_unit!(MMPerSecond3);

        /// Represents Newtons
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct Newtons(pub f32);
        syunit::basic_unit!(Newtons, "N");
        syunit::additive_unit!(Newtons);
        syunit::derive_units!(Newtons, Gramms, MMPerSecond2);   // Lazy implementation of the derive-units state

        /// Represents 
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct Gramms(pub f32);
        syunit::basic_unit!(Gramms, "g");
        syunit::additive_unit!(Gramms);
    // 

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
        type Inertia = Gramms;
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
        syunit::derive_units!(NewtonMeters, KgMeter2, RadPerSecond2);   // Lazy implementation of the derive-units state

        /// Represents a second moment of inertia in Kilogramms times meters squared
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct KgMeter2(pub f32);
        syunit::basic_unit!(KgMeter2, "kgm^2");
        syunit::additive_unit!(KgMeter2);
    // 

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



// //
// //
// //
//     /// Represents metric meters
//     #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
//     #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
//     pub struct Meter(pub f32);
//     syunit::basic_unit!(Meter, "m");

// //