use reverse_words::reverse;

fn main() {
    println!("Reverse Words!");

    let result = reverse("RustLang");
    assert_eq!(result, "gnaLtsuR");
}
