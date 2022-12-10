//! # Ulangan Rust #2
//!
//! Kamu harus buat sendiri fungsi `replace` yg sesuai utk kasus tertentu.
//! Rust library menyediakan beragam method pendukung.
//!

//!
//! Detail `replace` _string_ ada pada modul [`std::str`](https://doc.rust-lang.org/std/primitive.str.html#method.replace)
//!

/// # Replace characters and/or text in a string
///
/// You can now replace characters or text in your string
///
/// ```rust
/// use replace_in_string::replace_text;
///
/// let result_text = replace_text("Hello world", "world", "rust");
/// assert_eq!(result_text, "Hello rust");
///
/// let result_char = replace_text("Hello world", "e", "a");
/// assert_eq!(result_char, "Hallo world");
/// ```
pub fn replace_text(src: &str, from: &str, to: &str) -> String {
    src.replace(from, to)
}

/// # Replace a character in a string
///
/// You can replace ONLY a character in your string
///
/// ```rust
/// use replace_in_string::replace_char;
///
/// let result = replace_char("RustLang", 'u', 'e');
/// assert_eq!(result, "RestLang");
/// ```
pub fn replace_char(src: &str, from: char, to: char) -> String {
    let mut res = String::new();

    for i in src.chars() {
        match i {
            c if c == from => res.push(to),
            _ => res.push(i)
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected_char() {
        assert_eq!(replace_char("RustLang", 'u', 'e'), "RestLang");
    }

    #[test]
    fn returns_expected_text() {
        assert_eq!(replace_text("Hello world", "world", "rust"), "Hello rust");
        assert_eq!(replace_text("RustLang", "u", "e"), "RestLang");
    }
}