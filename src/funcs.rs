use core::ops::{Add, Sub};

/// Add two arrays of units
/// 
/// # Example
/// 
/// A simple example would be adding some relative distances to some component distances
/// 
/// ```rust
/// use syunit::prelude::*;
/// 
/// let abs_pos_list = [ PositionMM(2.0), PositionMM(1.0), PositionMM(-3.5) ];
/// let rel_dists = [ Millimeters(1.2), Millimeters(3.5), Millimeters(0.5) ];
/// 
/// assert!(compare_unit_arrays(add_unit_arrays(abs_pos_list, rel_dists), [ PositionMM(3.2), PositionMM(4.5), PositionMM(-3.0) ]));
/// ```
pub fn add_unit_arrays<U, Rhs, const C : usize>(base : [U; C], rhs : [Rhs; C]) -> [U::Output; C]
where
    U : Add<Rhs>,
    U : Copy,
    Rhs : Copy
{
    // Safe as zeroed, as it will not be read before being set
    let mut result : [U::Output; C] = unsafe { core::mem::zeroed() };

    for i in 0 .. C {
        result[i] = base[i] + rhs[i];
    }

    result
}

/// Subtract two arrays of units
/// 
/// # Example
/// 
/// A simple example would be subtracting some relative distances to some components distances
/// 
/// ```rust
/// use syunit::prelude::*;
/// 
/// let abs_pos_list = [ PositionMM(2.2), PositionMM(1.0), PositionMM(-3.5) ];
/// let rel_dists = [ Millimeters(1.2), Millimeters(3.5), Millimeters(0.5) ];
/// 
/// assert!(compare_unit_arrays(sub_unit_arrays(abs_pos_list, rel_dists), [ PositionMM(1.0), PositionMM(-2.5), PositionMM(-4.0) ]));
/// ```
pub fn sub_unit_arrays<U, Rhs, const C : usize>(base : [U; C], rhs : [Rhs; C]) -> [U::Output; C]
where
    U : Sub<Rhs>,
    U : Copy,
    Rhs : Copy
{
    // Safe as zeroed, as it will not be read before being set
    let mut result : [U::Output; C] = unsafe { core::mem::zeroed() };

    for i in 0 .. C {
        result[i] = base[i] - rhs[i];
    }

    result
}

/// Compare two unit arrays
/// 
/// # Example
/// 
/// ```rust
/// use syunit::prelude::*;
/// 
/// let pos_list = [ PositionMM(2.0), PositionMM(1.0), PositionMM(-3.5) ];
/// let pos_list_eq = [ PositionMM(2.0), PositionMM(1.0), PositionMM(-3.5) ];
/// let pos_list_uneq = [ PositionMM(1.0), PositionMM(2.0), PositionMM(3.5) ];
/// 
/// assert!(compare_unit_arrays(pos_list, pos_list_eq));
/// assert!(!compare_unit_arrays(pos_list, pos_list_uneq)); 
/// ```
pub fn compare_unit_arrays<U, const C : usize>(base : [U; C], rhs : [U; C]) -> bool
where
    U : PartialEq,
    U : Copy
{
    for i in 0 .. C {
        if base[i] != rhs[i] {
            return false;
        }
    }

    true
}