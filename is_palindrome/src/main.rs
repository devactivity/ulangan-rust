use is_palindrome::Algorithms;

fn main() {
    let s = String::from("radar");
    println!("Is '{}' a palindrome? {}", s, (&s).is_palindrome());
    let num = 12321;
    println!("Is '{}' a palindrome? {}", num, (&num).is_palindrome());
}
