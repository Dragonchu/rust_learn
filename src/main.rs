mod collection_basic;
use crate::collection_basic::vector_basic;
use crate::collection_basic::string_basic;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let degree = &args[1];
    let domain = &args[2];

    if degree.eq("basic")  {
        if domain.eq("string") {
            string_basic::basic::example();
        } else if domain.eq("vector") {
            vector_basic::basic::example();
        }else{
            println!("no matched domain");
        }
    }else{
        println!("no matched degree");
    }
}
