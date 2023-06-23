fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn main() {

    let my_string = String::from("hello world");

    let _word = first_word(&my_string[..]);

    let my_literal_string = "hello world";

    let _word = first_word(my_literal_string);

}
