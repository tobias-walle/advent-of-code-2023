use num_traits::{CheckedMul, Num, Signed};

/// Compute the greatest common denominator
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Num + Signed + Copy,
{
    if b.is_zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Compute the least common multiple
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Num + Copy + Signed + CheckedMul,
{
    (a / gcd(a, b)).checked_mul(&b).expect("Number overflow")
}
