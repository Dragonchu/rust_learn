mod collection_basic;
mod error_handling;
use crate::collection_basic::string_basic;
use crate::collection_basic::vector_basic;
use crate::error_handling::use_result::result_usage;

use std::env;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let degree = &args[1];
    let domain = &args[2];

    if degree.eq("basic") {
        if domain.eq("string") {
            string_basic::basic::example();
        } else if domain.eq("vector") {
            vector_basic::basic::example();
        } else if domain.eq("error_handling") {
            result_usage::read_file_example();
        } else {
            println!("no matched domain");
        }
    } else {
        println!("no matched degree");
    }
}
