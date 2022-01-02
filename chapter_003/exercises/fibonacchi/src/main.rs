use std::io;
use std::str::FromStr;

fn main() {
    println!("Welcome to the Fibonacci calculator!");
    let limit = read_input_number::<i32>("Please enter your fibonacci limit");
    println!("Fibonacci number: {}", calculate_fibonacci(limit));
}

fn read_input_number<T: std::str::FromStr>(msg: &str) -> T where <T as FromStr>::Err: std::fmt::Display {
    loop{
        let mut input = String::new();
        println!("{}", msg);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        match input.trim().parse(){
            Ok(num) => {
                break num;
            },
            Err(err) => {
                println!("Encountered an error: {}", err);
                continue;
            }
        }
    }
}

fn calculate_fibonacci(limit: i32) -> i64 {
    let mut current_one = 0;
    let mut current_two = 0;
    for _number in 0..limit{
        if current_one == 0 && current_two == 0{
            current_two = 1;
        }else if current_one == 0 && current_two == 1{
            current_one = 1;
            current_two = 1;
        }else{
            let temp = current_one;
            current_one = current_two;
            current_two = temp + current_one;
        }
    }
    current_two
}