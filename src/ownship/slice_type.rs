pub fn example() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    println!("word: {word}, s: {s}");
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    println!("slice show");
    slice_show();

    let s2 = String::from("hello world");
    let return_slice = first_word_return_slice(&s2);
    println!("return_slice: {return_slice}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_return_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn slice_show() {
    let s = String::from("Hello, slice");
    let hello = &s[0..5];
    let slice = &s[7..12];
    println!("hello: {hello}, slice: {slice}");

    let example1 = &s[3..];
    let len = s.len();
    let example2 = &s[..4];
    let full = &s[..];
    println!("example1: {example1}, example2: {example2}, full: {full}");
}
