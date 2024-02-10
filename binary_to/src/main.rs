use binary_to::BinaryConverter;

fn main() {
    assert_eq!(BinaryConverter::binary_to_decimal("0000000110"), Some(4));
    assert_eq!(BinaryConverter::binary_to_hexadecimal("00110110"), "0x36");
}
