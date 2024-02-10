pub struct BinaryConverter;

impl BinaryConverter {
    pub fn binary_to_decimal(binary: &str) -> Option<u128> {
        if binary.len() > 128 {
            return None;
        }
        let mut num: u128 = 0;
        for c in binary.chars() {
            num = match c {
                '1' => num.checked_mul(2)?.checked_add(1)?,
                '0' => num.checked_mul(2)?,
                _ => return None,
            };
        }
        Some(num)
    }

    pub fn binary_to_hexadecimal(binary_str: &str) -> String {
        const HEX: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        let binary_str = binary_str
            .trim()
            .chars()
            .filter(|c| *c != ' ')
            .collect::<String>();

        if binary_str.is_empty()
            || binary_str.len() % 4 != 0
            || binary_str.chars().any(|c| c != '0' && c != '1')
        {
            return "Invalid input".to_string();
        }

        let hexadecimal = binary_str
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| {
                let bits = chunk
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0_u8, |acc, (i, &bit)| acc | ((bit as u8 - b'0') << i));

                HEX[bits as usize]
            })
            .collect::<String>();

        format!("0x{}", hexadecimal)
    }
}
