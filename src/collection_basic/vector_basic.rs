pub mod basic {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    pub fn example() {
        println!("Creating a new, empty vector to hold values of type i32");
        let v: Vec<i32> = Vec::new();
        println!("{:?}", v);

        println!("Creating a new vector containing values");
        let v = vec![1, 2, 3];
        println!("{:?}", v);

        println!("Using the push method to add values to a vector");
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        println!("{:?}", v);

        println!("Using indexing syntax or the get method to access an item in a vector");
        let v = vec![1, 2, 3, 4, 5];
        println!("{:?}", v);
        let third: &i32 = &v[2];
        println!("The third element is {third}");
        let third = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element"),
        }
        let tenth = v.get(9);
        match tenth {
            Some(tenth) => println!("The tenth element is {tenth}"),
            None => println!("There is no tenth element"),
        }

        println!(
            "Printing each element in a vector by iterating over the elements using a for loop"
        );
        let v = vec![4, 5, 6];
        for i in &v {
            println!("{i}");
        }

        println!("Iterating over mutable references to elements in a vector");
        let mut v = vec![32, 45, 67];
        println!("{:?}", v);
        for i in &mut v {
            *i += 50;
        }
        println!("{:?}", v);

        println!("Defining an enum to store values of different types in one vector");
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        println!("{:?}", row);
    }
}
