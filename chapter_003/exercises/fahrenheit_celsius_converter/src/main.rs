use std::io;
fn main() {
    println!("Welcome to fahrenheit to celsius converter");
    let fahrenheit = read_input_number("Please enter the fahrenheit");

    println!("F:{} is C:{}", fahrenheit, fahrenheit_to_celsius(fahrenheit));
}

fn fahrenheit_to_celsius(f: f32) -> f32{
    (f-32.0)*5.0/9.0
}

fn read_input_number(msg: &str) -> f32 {
    let input_verified: f32;
    loop{
        let mut input = String::new();
        println!("{}", msg);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let input: f32 = match input.trim().parse(){
            Ok(num) => {
                num
            },
            Err(err) => {
                println!("Encountered an error: {}", err);
                continue;
            }
        };
        // println!("This one is not reached on error because of the continue");
        input_verified = input;
        break;
    }

    input_verified
}
