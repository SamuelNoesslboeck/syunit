// ##########################
// #    Operation macros    #
// ##########################
    /// Automatically implements multiplication between three units
    /// 
    /// ### Macro buildup
    /// 
    /// ```rust, ignore
    /// ( $input:ident, $by:ident, $output:ident ) 
    /// ```
    /// 
    /// Takes three unit types and implements
    /// 
    /// ```rust, ignore
    /// input * by = output
    /// ```
    /// 
    /// Optionally a conversion (a [f32] literal) can be added: 
    /// 
    /// ```rust, ignore
    /// ( $input:ident, $by:ident, $output:ident, $conv:literal ) 
    /// ```
    /// 
    /// resulting in:
    /// 
    /// ```rust, ignore
    /// input * by * conv = output
    /// ```
    #[macro_export]
    macro_rules! impl_mul {
        ( $input:ident, $by:ident, $output:ident ) => {
            impl core::ops::Mul<$by> for $input {
                type Output = $output;

                #[inline]
                fn mul(self, rhs : $by) -> $output {
                    $output(self.0 * rhs.0)
                }
            }
        };
        ( $input:ident, $by:ident, $output:ident, $conv:literal ) => {
            impl core::ops::Mul<$by> for $input {
                type Output = $output;

                #[inline]
                fn mul(self, rhs : $by) -> $output {
                    $output(self.0 * rhs.0 * $conv)
                }
            }
        };
    }

    /// Identical to [impl_mul!], however it implements the division in both directions, so
    /// 
    /// ```rust, ignore
    /// input * by = output
    /// ```
    /// 
    /// and 
    /// 
    /// ```rust, ignore
    /// by * input = output
    /// ```
    #[macro_export]
    macro_rules! impl_mul_bidir {
        ( $input:ident, $by:ident, $output:ident ) => {
            syunit::impl_mul!( $input, $by, $output );
            syunit::impl_mul!( $by, $input, $output );
        }; 
        ( $input:ident, $by:ident, $output:ident, $conv:literal ) => {
            syunit::impl_mul!( $input, $by, $output, $conv );
            syunit::impl_mul!( $by, $input, $output, $conv );
        };
    }

    /// Automatically implements division between three units
    /// 
    /// ### Macro buildup
    /// 
    /// ```rust, ignore
    /// ( $input:ident, $by:ident, $output:ident ) 
    /// ```
    /// 
    /// Takes three unit types and implements
    /// 
    /// ```rust, ignore
    /// input / by = output
    /// ```
    /// 
    /// Optionally a conversion (a [f32] literal) can be added: 
    /// 
    /// ```rust, ignore
    /// ( $input:ident, $by:ident, $output:ident, $conv:literal ) 
    /// ```
    /// 
    /// resulting in:
    /// 
    /// ```rust, ignore
    /// input / by / conv = output
    /// ```
    #[macro_export]
    macro_rules! impl_div {
        ( $input:ident, $by:ident, $output:ident ) => {
            impl core::ops::Div<$by> for $input {
                type Output = $output;

                #[inline]
                fn div(self, rhs : $by) -> $output {
                    $output(self.0 / rhs.0)
                }
            }
        };
        ( $input:ident, $by:ident, $output:ident, $conv:literal ) => {
            impl core::ops::Div<$by> for $input {
                type Output = $output;

                #[inline]
                fn div(self, rhs : $by) -> $output {
                    $output(self.0 / rhs.0 / $conv)
                }
            }
        };
    }

    /// Identical to [impl_div!], however it implements the division in both directions, so
    /// 
    /// ```rust, ignore
    /// input / by = output
    /// ```
    /// 
    /// and 
    /// 
    /// ```rust, ignore
    /// input / output = by
    /// ```
    #[macro_export]
    macro_rules! impl_div_bidir {
        ( $input:ident, $by:ident, $output:ident ) => {
            syunit::impl_div!( $input, $by, $output );
            syunit::impl_div!( $input, $output, $by );
        }; 
        ( $input:ident, $by:ident, $output:ident, $conv:literal ) => {
            syunit::impl_div!( $input, $by, $output, $conv );
            syunit::impl_div!( $input, $output, $by, $conv );
        };
    }
//

// ####################
// #    Conversion    #
// ####################
    // TODO: Improve documentation
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
    macro_rules! impl_conversion {
        ( $input:ident, $output:ident ) => {
            impl From<$input> for $output {
                #[inline(always)]
                fn from(value : $input) -> Self {
                    Self(value.0)
                }
            }

            impl From<$output> for $input {
                #[inline(always)]
                fn from(value : $output) -> Self {
                    Self(value.0)
                }
            }
        };
        ( $input:ident, $output:ident, $conv:literal ) => {
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

    /// Implements everything required to form a "derive over time like"-connection between the given units
    #[macro_export]
    macro_rules! impl_full_conversion {
        ( $input:ident, $by:ident, $output:ident ) => {
            syunit::impl_mul_bidir!( $input, $by, $output );
            syunit::impl_div_bidir!( $output, $by, $input );
        };
        ( $input:ident, $by:ident, $output:ident, $conv:literal ) => {
            syunit::impl_mul_bidir!( $input, $by, $output, $conv );
            syunit::impl_div_bidir!( $output, $by, $input, $conv );
        };
    }
//

// ####################
// #    Basic unit    #
// ####################
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

            // Factor
                impl core::ops::Mul<syunit::Factor> for $a {
                    type Output = $a;

                    #[inline]
                    fn mul(self, rhs : syunit::Factor) -> Self::Output {
                        Self(self.0 * rhs.as_f32())
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
//

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

/// Helper macro for position units
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
                Self(self.0 - rhs.0)
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

        syunit::impl_conversion!($pos, $unit);
    };
}

// TODO: Improve documentation to cover matching arms
/// Implements everything required to form a "derive over time like"-connection between the given units
#[macro_export]
macro_rules! derive_units {
    ( $dist:ident, $vel:ident, $time:ident ) => {

        syunit::impl_full_conversion!( $vel, $time, $dist );

        impl syunit::DerivableUnit<$time> for $dist { 
            type Result = $vel;
        }

        impl syunit::IntegrableUnit<$time> for $vel { 
            type Result = $dist;
        }
    };
}

/// Automatically implement [InertiaUnit](crate::InertiaUnit) for the given unit
/// 
#[macro_export]
macro_rules! inertia_unit {
    ( $name:ident, $length:ident, $reduced:ident ) => {
        impl syunit::InertiaUnit<$length> for $name {
            type Reduced = $reduced;
        }
    };
    ( $name:ident, $length:ident, $reduced:ident, $conv:literal ) => {
        impl syunit::InertiaUnit<$length> for $name {
            type Reduced = $reduced;

            fn reduce(self, ratio : $length) -> Self::Reduced {
                Self::Reduced::from(<Self as Into<f32>>::into(self) * <$length as Into<f32>>::into(ratio) * <$length as Into<f32>>::into(ratio) * $conv)
            }

            fn extend(reduced : Self::Reduced, ratio : $length) -> Self {
                Self::from(<$reduced as Into<f32>>::into(reduced) / <$length as Into<f32>>::into(ratio) / <$length as Into<f32>>::into(ratio) / $conv)
            }
        }
    };
}