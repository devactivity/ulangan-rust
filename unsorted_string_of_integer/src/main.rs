use unsorted_string_of_integer::mystery_range;

fn main() {
    let data = mystery_range("1568141291110137", 10);
    // 15 6 8 14 12 9 11 10 13 7
    // 6 to 15

    assert_eq!(data, (6, 15));
    assert_eq!(mystery_range("6291211413114538107", 14), (1, 15));
    assert_eq!(mystery_range("13161820142119101112917232215", 15), (9, 23));
    assert_eq!(
        mystery_range("2318134142120517221910151678611129", 20),
        (4, 23)
    );
}
