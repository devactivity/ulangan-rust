use std::collections::HashMap;

struct CharCounts {
    counts: HashMap<usize, usize>,
}

impl CharCounts {
    fn from_str(s: &str) -> Self {
        let mut counts = HashMap::new();
        for c in s.chars() {
            if let Some(digit) = c.to_digit(10) {
                *counts.entry(digit as usize).or_insert(0) += 1;
            }
        }

        CharCounts { counts }
    }

    fn matches_range(&self, start: u32, end: u32) -> bool {
        let mut range_counts = HashMap::new();
        for i in start..=end {
            let mut num = i;
            while num > 0 {
                let digit = (num % 10) as usize;
                *range_counts.entry(digit).or_insert(0) += 1;
                num /= 10;
            }
        }
        self.counts == range_counts
    }
}

fn contains_digit(s: &str, digit: u32) -> bool {
    s.contains(&digit.to_string())
}

pub fn mystery_range(s: &str, n: usize) -> (u32, u32) {
    let counts = CharCounts::from_str(s);

    for i in 1..=100_u32 {
        let j = i + n as u32 - 1;

        if counts.matches_range(i, j) && contains_digit(s, i) && contains_digit(s, j) {
            return (i, j);
        }
    }

    unreachable!()
}
