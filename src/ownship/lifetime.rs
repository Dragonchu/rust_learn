#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn example() {
    print!("lifetime example start.\n");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    print!("{:?}\n", i);
    print!("lifetime example end.\n");
}