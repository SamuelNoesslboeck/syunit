/// Helper macro that implements everything needed to do +,-,+=,-= operations with the unit itself
#[macro_export]
macro_rules! additive_unit {
    ( $unit:ident ) => {
        impl core::ops::Add<$unit> for $unit {
            type Output = $unit;
        
            #[inline(always)]
            fn add(self, rhs: $unit) -> Self::Output {
                $unit(self.0 + rhs.0)
            }
        }

        impl core::ops::AddAssign<$unit> for $unit {
            #[inline(always)]
            fn add_assign(&mut self, rhs: $unit) {
                self.0 += rhs.0;
            }
        }        
        
        impl core::ops::Sub<$unit> for $unit {
            type Output = $unit;
        
            #[inline(always)]
            fn sub(self, rhs: $unit) -> Self::Output {
                $unit(self.0 - rhs.0) 
            }
        }

        impl core::ops::SubAssign<$unit> for $unit {
            #[inline]
            fn sub_assign(&mut self, rhs : $unit) {
                self.0 -= rhs.0
            }
        }
    };
}

/// Implements the basics for a unit
#[macro_export]
macro_rules! basic_unit {
    ( $a:ident ) => {      
        impl $a {
            /// Zero value of this unit (0.0)
            pub const ZERO : Self = Self(0.0);
            /// Positive Infinity value of this unit (f32::INFINITY)
            pub const INFINITY : Self = Self(f32::INFINITY);
            /// Negative Infinity value of this unit (f32::INFINITY)
            pub const NEG_INFINITY : Self = Self(f32::NEG_INFINITY);
            /// NaN value of this unit (f32::NAN)
            pub const NAN : Self = Self(f32::NAN);

            /// Returns the absolute value of the unit 
            #[inline(always)]
            pub fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns `true` if this units value is neither NaN nor Infinite
            #[inline(always)]
            pub fn is_finite(self) -> bool {
                self.0.is_finite()
            }

            /// Returns `true` if this units value is neither NaN, Infinite or zero
            #[inline(always)]
            pub fn is_normal(self) -> bool {
                self.0.is_normal()
            }

            /// Returns `true` if this units value is Nan
            #[inline(always)]
            pub fn is_nan(self) -> bool {
                self.0.is_nan()
            }

            /// Returns the unit raised to the given integer power `pow`
            #[inline(always)]
            pub fn powi(self, pow : i32) -> Self {
                Self(self.0.powi(pow))
            }

            /// Returns the unit raised to the given power `pow`
            #[inline(always)]
            pub fn powf(self, pow : f32) -> Self {
                Self(self.0.powf(pow))
            }

            /// Returns the sin of this units value
            #[inline(always)]
            pub fn sin(self) -> f32 {
                self.0.sin()
            }

            /// Returns the cos of this units value
            #[inline(always)]
            pub fn cos(self) -> f32 {
                self.0.tan()
            }

            /// Returns the tan of this units value
            #[inline(always)]
            pub fn tan(self) -> f32 {
                self.0.tan()
            }

            /// Returns `true` if the sign bit of this value is negative (value smaller than 0.0, -0.0 included)
            pub fn is_sign_negative(self) -> bool { 
                self.0.is_sign_negative()
            }

            /// Returns `true` if the sign bit of this value is positive (value smaller than 0.0, -0.0 included)
            pub fn is_sign_positive(self) -> bool {
                self.0.is_sign_positive()
            }

            /// Returns the smaller value of this and another unit
            #[inline(always)]
            pub fn min(self, other : Self) -> Self {
                Self(self.0.min(other.0))
            }

            /// Return the bigger value of this and another unit
            #[inline(always)]
            pub fn max(self, other : Self) -> Self {
                Self(self.0.max(other.0))
            }
            
            /// Return the bigger value of this and another unit, working with references
            #[inline(always)]
            pub fn max_ref<'a>(&'a self, other : &'a Self) -> &'a Self {
                if *self < *other {
                    other
                } else {
                    self
                }
            }

            /// Return the bigger value of this and another unit, working with references
            #[inline(always)]
            pub fn min_ref<'a>(&'a self, other : &'a Self) -> &'a Self {
                if *self > *other {
                    other
                } else {
                    self
                }
            }
        }

        impl syunit::Unit for $a { 
            fn new(v : f32) -> Self {
                Self(v)
            }
        }

        impl core::str::FromStr for $a {
            type Err = <f32 as core::str::FromStr>::Err;
        
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(s.parse::<f32>()?))
            }
        }

        impl core::fmt::Display for $a {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl core::fmt::Debug for $a {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($a), self.0))
            }
        }

        impl core::convert::Into<f32> for $a {
            #[inline(always)]
            fn into(self) -> f32 {
                self.0
            }
        }

        // Ref
            
        // 

        // Negation
            impl core::ops::Neg for $a {
                type Output = Self;
                
                #[inline(always)]
                fn neg(self) -> Self::Output {
                    Self(-self.0)
                }
            }
        //

        // Multiplication
            impl core::ops::Mul<f32> for $a {
                type Output = $a;
                
                #[inline(always)]
                fn mul(self, rhs: f32) -> Self::Output {
                    $a(self.0 * rhs)
                }
            }

            impl core::ops::Mul<$a> for f32 {
                type Output = $a;

                #[inline(always)]
                fn mul(self, rhs : $a) -> Self::Output {
                    $a(self * rhs.0)
                }
            }
        // 
        
        // Division
            impl core::ops::Div<f32> for $a {
                type Output = $a;
            
                #[inline(always)]
                fn div(self, rhs: f32) -> Self::Output {
                    $a(self.0 / rhs)
                }
            }

            impl core::ops::Div<$a> for $a {
                type Output = f32;

                #[inline(always)]
                fn div(self, rhs : $a) -> Self::Output {
                    self.0 / rhs.0
                }
            }
        // 
    };
}

/// Implements everything required to form a "derive over time like"-connection between the given units
#[macro_export]
macro_rules! derive_units {
    ( $dist:ident, $vel:ident, $time:ident ) => {
        impl core::ops::Mul<$time> for $vel {
            type Output = $dist;
        
            #[inline(always)]
            fn mul(self, rhs: $time) -> Self::Output {
                $dist(self.0 * rhs.0)
            }
        }

        impl core::ops::Mul<$vel> for $time {
            type Output = $dist;
            
            #[inline(always)]
            fn mul(self, rhs: $vel) -> Self::Output {
                $dist(self.0 * rhs.0)
            }
        }

        impl core::ops::Div<$vel> for $dist {
            type Output = $time;
        
            #[inline(always)]
            fn div(self, rhs: $vel) -> Self::Output {
                $time(self.0 / rhs.0)
            }
        }

        impl core::ops::Div<$time> for $dist {
            type Output = $vel;
        
            #[inline(always)]
            fn div(self, rhs: $time) -> Self::Output {
                $vel(self.0 / rhs.0)
            }
        }
    };
}