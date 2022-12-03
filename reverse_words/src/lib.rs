//! # Ulangan Rust #1
//! Dasar fungsi `reverse` adalah sebagai berikut:

//! ```rust
//! let x = "    a  b c".to_string();
//! let v: Vec<_> = x.split(" ").collect();
//! assert_eq!(v, &["", "", "", "", "a", "", "b", "c"]);
//! ```

/// # Reverse words in Rust
/// Kamu bisa _me-reverse words_ menggunakan fungsi dibawah ini
///
/// ```
/// use reverse_words::reverse;
///
/// let result = reverse("RustLang");
/// assert_eq!(result, "gnaLtsuR");
/// ```
pub fn reverse(s: &str) -> String {
    s.to_string()
        .split(" ")
        .map(| e | e.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual_test() {
        let word = "RustLang";
        println!("original: {}, reverse: {}", word, reverse(&word));

        assert_eq!("gnaLtsuR", reverse(&word));
    }

    #[test]
    fn random_test() {
        let words = ["a b c d e f", "My Name is", "RustLang", " ", "Belajar Rust", "Ulangan Rust"];

        for i in 0..words.len() {
            let test_str = words[i];
            println!("original: {}, reverse: {}", test_str, reverse(&test_str));

            assert_eq!(reverse(&test_str), reverse(&test_str));
        }
    }
}