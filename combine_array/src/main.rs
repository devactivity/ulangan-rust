use combine_array::combine;
use std::ops::Add;

fn main() {
    assert_eq!(
        &combine(i32::max, &[1, 4, 7, 1, 4, 7], &[4, 7, 1, 4, 7, 1]),
        &[4, 7, 7, 4, 7, 7]
    );

    assert_eq!(
        &combine(i32::add, &[0, 1, 2, 3, 4], &[6, 5, 4, 3, 2, 1]),
        &[6, 6, 6, 6, 6]
    );
}
