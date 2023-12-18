pub mod basic{
    pub fn initialize_example(){
        println!("Creating a new, empty vector to hold values of type i32");
        let v: Vec<i32> = Vec::new();
        println!("{:?}",v);
    }
}