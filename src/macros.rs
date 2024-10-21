/// Implements the basics for a unit
#[macro_export]
macro_rules! basic_unit_helper {
    ( $a:ident ) => {      
        // Display traits
            impl core::str::FromStr for $a {
                type Err = <f32 as core::str::FromStr>::Err;
            
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(Self(s.parse::<f32>()?))
                }
            }

            impl core::fmt::Debug for $a {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_fmt(format_args!("{}({})", stringify!($a), self.0))
                }
            }

            impl core::convert::From<f32> for $a {
                #[inline(always)]
                fn from(value : f32) -> Self {
                    Self(value)
                }
            }

            impl core::convert::Into<f32> for $a {
                #[inline(always)]
                fn into(self) -> f32 {
                    self.0
                }
            }
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

        impl syunit::Unit for $a { 
            /// Zero value of this unit (0.0)
            const ZERO : Self = Self(0.0);
            /// Positive Infinity value of this unit (f32::INFINITY)
            const INFINITY : Self = Self(f32::INFINITY);
            /// Negative Infinity value of this unit (f32::INFINITY)
            const NEG_INFINITY : Self = Self(f32::NEG_INFINITY);
            /// NaN value of this unit (f32::NAN)
            const NAN : Self = Self(f32::NAN);
        }
    };
}

/// Implements the basics for a unit
#[macro_export]
macro_rules! basic_unit {
    ( $name:ident ) => {
        syunit::basic_unit_helper!( $name );

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <f32 as core::fmt::Display>::fmt(&self.0, f)
            }
        }
    };
    ( $name:ident, $sym:literal ) => {
        syunit::basic_unit_helper!( $name );

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_fmt(format_args!("{}{}", self.0, $sym))
            }
        }
    }
}

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

        impl syunit::AdditiveUnit for $unit { }
    };
}

#[macro_export]
macro_rules! position_unit {
    ( $pos:ident, $unit:ident ) => {
        impl core::ops::Add<$unit> for $pos {
            type Output = $pos;

            fn add(self, rhs: $unit) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl core::ops::AddAssign<$unit> for $pos {
            fn add_assign(&mut self, other : $unit) {
                self.0.add_assign(other.0);
            }
        }

        impl core::ops::Sub<$unit> for $pos {
            type Output = $pos;

            fn sub(self, rhs: $unit) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl core::ops::SubAssign<$unit> for $pos {
            fn sub_assign(&mut self, other : $unit) {
                self.0.sub_assign(other.0);
            }
        }

        impl core::ops::Sub<$pos> for $pos {
            type Output = $unit; 

            fn sub(self, rhs: $pos) -> Self::Output {
                $unit(self.0 - rhs.0)
            }
        }
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

        impl syunit::DerivableUnit<$vel, $time> for $dist { }

        impl syunit::IntegrableUnit<$dist, $time> for $vel { }
    };
}

/// Implements conversion between two units
/// 
/// ### Syntax
/// 
/// `( input, output, conv )`
/// 
/// - `input`: The input unit
/// - `output`: The output unit
/// - `conv`: The conversion factor, can be an expression (=> `output = input * conv`)
#[macro_export]
macro_rules! conversion_unit {
    ( $input:ident, $output:ident, $conv:expr ) => {
        impl From<$input> for $output {
            #[inline(always)]
            fn from(value : $input) -> Self {
                Self(value.0 * ($conv))
            }
        }

        impl From<$output> for $input {
            #[inline(always)]
            fn from(value : $output) -> Self {
                Self(value.0 / ($conv))
            }
        }
    };
}