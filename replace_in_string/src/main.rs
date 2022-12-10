use replace_in_string::{replace_char, replace_text};

fn main() {
    assert_eq!(replace_char("My Name is", 'i', 'u'), "My Name us");
    assert_eq!(replace_text("My Name is", "Name is", "Age"), "My Age");
}
