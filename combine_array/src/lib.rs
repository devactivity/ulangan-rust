//! # Ulangan Rust #7
//!
//! Combine Arrays dengan memanfaatkan `function/closure`. Memanfaatkan pendekatan Trait Bounds dan Generic type.
//!

/// # Combine Arrays
///
/// You can use many types on the element.
///
/// ```rust
/// use combine_array::combine;
/// use std::ops::{Add, Sub};
///
/// assert_eq!(&combine(i32::add, &[0,1,2,3],     &[0,1,2,3]),     &[0,2,4,6]);
/// assert_eq!(&combine(i32::add, &[0,1,2,3,4  ], &[6,5,4,3,2,1]), &[6,6,6,6,6  ]);
/// assert_eq!(&combine(i32::max, &[1,4,7,1,4,7], &[4,7,1,4,7,1]), &[4,7,7,4,7,7]);
/// assert_eq!(&combine(i32::sub, &[0,1,2,3,4,5], &[6,5,4,3,2,1]), &[-6,-4,-2,0,2,4] );
/// assert_eq!(&combine(i32::pow, &[10; 10],    &[0,1,2,3,4,5,6]), &[1,10,100,1000,10000,100000,1000000]);
/// ```
pub fn combine<F, T, S, R>(f: F, arr1: &[T], arr2: &[S]) -> Vec<R>
where
    F: Fn(T, S) -> R,
    T: Copy,
    S: Copy,
{
    let min_len = arr1.len().min(arr2.len());
    let mut result = Vec::with_capacity(min_len);

    for i in 0..min_len {
        let x = arr1[i];
        let y = arr2[i];

        result.push(f(x, y));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::{Add, Sub};

    #[test]
    fn returns_expected_result() {
        assert_eq!(
            &combine(i32::add, &[0, 1, 2, 3], &[0, 1, 2, 3]),
            &[0, 2, 4, 6]
        );
        assert_eq!(
            &combine(i32::add, &[0, 1, 2, 3, 4], &[6, 5, 4, 3, 2, 1]),
            &[6, 6, 6, 6, 6]
        );
        assert_eq!(
            &combine(i32::max, &[1, 4, 7, 1, 4, 7], &[4, 7, 1, 4, 7, 1]),
            &[4, 7, 7, 4, 7, 7]
        );
        assert_eq!(
            &combine(i32::sub, &[0, 1, 2, 3, 4, 5], &[6, 5, 4, 3, 2, 1]),
            &[-6, -4, -2, 0, 2, 4]
        );
        assert_eq!(
            &combine(i32::pow, &[10; 10], &[0, 1, 2, 3, 4, 5, 6]),
            &[1, 10, 100, 1000, 10000, 100000, 1000000]
        );
    }
}
