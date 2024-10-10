use core::ops::{Add, Sub};

/// Add two arrays of units
/// 
/// # Example
/// 
/// A simple example would be adding some relative distances to some component distances
/// 
/// ```rust
/// use syunit::*;
/// 
/// let abs_pos_list = [ AbsPos(2.0), AbsPos(1.0), AbsPos(-3.5) ];
/// let rel_dists = [ RelDist(1.2), RelDist(3.5), RelDist(0.5) ];
/// 
/// assert!(compare_unit_arrays(add_unit_arrays(abs_pos_list, rel_dists), [ AbsPos(3.2), AbsPos(4.5), AbsPos(-3.0) ]));
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
/// use syunit::*;
/// 
/// let abs_pos_list = [ AbsPos(2.2), AbsPos(1.0), AbsPos(-3.5) ];
/// let rel_dists = [ RelDist(1.2), RelDist(3.5), RelDist(0.5) ];
/// 
/// assert!(compare_unit_arrays(sub_unit_arrays(abs_pos_list, rel_dists), [ AbsPos(1.0), AbsPos(-2.5), AbsPos(-4.0) ]));
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
/// use syunit::*;
/// 
/// let comp_pos_ori = [ AbsPos(2.0), AbsPos(1.0), AbsPos(-3.5) ];
/// let comp_pos_same = [ AbsPos(2.0), AbsPos(1.0), AbsPos(-3.5) ];
/// let comp_pos_diff = [ AbsPos(1.0), AbsPos(2.0), AbsPos(3.5) ];
/// 
/// assert!(compare_unit_arrays(comp_pos_ori, comp_pos_same));
/// assert!(!compare_unit_arrays(comp_pos_ori, comp_pos_diff)); 
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