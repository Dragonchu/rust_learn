pub mod basic {
    pub fn example() {
        println!("Finding the largest number in a list of numbers");
        let number_list = vec![34, 50, 25, 100, 65];
        find_largest_number(&number_list);
        println!("{:?}", number_list);
    }

    fn find_largest_number(number_list: &Vec<i32>) {
        let mut largest = &number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }
}
