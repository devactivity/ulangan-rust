//! Ulangan Rust #5
//! Membuat program Palindrome dengan Trait

/// # Palindrome
/// Program Algoritma: Palindrome
///
/// ```
/// use is_palindrome::Algorithms;
///
/// let s = "radar";
/// assert!((&s).is_palindrome());
/// ```

// with function
pub fn is_palindrome(s: &str) -> bool {
    let reversed_string = s.chars().rev().collect::<String>();
    s == reversed_string.as_str()
}

// with basic trait
trait Algorithm {
    fn is_palindrome(&self) -> bool;
}

impl Algorithm for String {
    fn is_palindrome(&self) -> bool {
        let reversed_string = self.chars().rev().collect::<String>();

        self.as_str() == reversed_string.as_str()
    }
}

// with generic trait
pub trait Algorithms<T> {
    fn is_palindrome(&self) -> bool;
}

impl<'a, T> Algorithms<T> for &'a T
where
    T: std::fmt::Display + Clone,
{
    fn is_palindrome(&self) -> bool {
        let string_representation = self.to_string();
        let reversed_string = string_representation.chars().rev().collect::<String>();

        string_representation == reversed_string
    }
}
