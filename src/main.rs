mod collection_basic;
mod error_handling;
use crate::collection_basic::string_basic;
use crate::collection_basic::vector_basic;
use crate::error_handling::use_result::result_usage;

use std::env;

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
