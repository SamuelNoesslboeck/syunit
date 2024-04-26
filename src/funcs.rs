use core::ops::{Add, Sub};

/// Add two arrays of units
/// 
/// # Example
/// 
/// A simple example would be adding some delta distances to some gamma distances
/// 
/// ```rust
/// use syunit::*;
/// 
/// let gammas = [ Gamma(2.0), Gamma(1.0), Gamma(-3.5) ];
/// let deltas = [ Delta(1.2), Delta(3.5), Delta(0.5) ];
/// 
/// assert!(compare_unit_arrays(add_unit_arrays(gammas, deltas), [ Gamma(3.2), Gamma(4.5), Gamma(-3.0) ]));
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
/// A simple example would be subtracting some delta distances to some gamma distances
/// 
/// ```rust
/// use syunit::*;
/// 
/// let gammas = [ Gamma(2.2), Gamma(1.0), Gamma(-3.5) ];
/// let deltas = [ Delta(1.2), Delta(3.5), Delta(0.5) ];
/// 
/// assert!(compare_unit_arrays(sub_unit_arrays(gammas, deltas), [ Gamma(1.0), Gamma(-2.5), Gamma(-4.0) ]));
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
/// let gammas_ori = [ Gamma(2.0), Gamma(1.0), Gamma(-3.5) ];
/// let gammas_same = [ Gamma(2.0), Gamma(1.0), Gamma(-3.5) ];
/// let gammas_diff = [ Gamma(1.0), Gamma(2.0), Gamma(3.5) ];
/// 
/// assert!(compare_unit_arrays(gammas_ori, gammas_same));
/// assert!(!compare_unit_arrays(gammas_ori, gammas_diff)); 
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