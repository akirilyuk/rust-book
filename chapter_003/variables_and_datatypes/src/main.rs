use std::io;

const MAX_POINTS: u32 = 100_000;

fn main() {
    let x = 5;
    let x = x +1;
    let x = x *2 ;
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", x);
    let spaces = "    ";
    let spaces = spaces.len();

    let x: f32 = 2.0;

    let sum = 5 +10;
    let difference = 10 - 9;
    let product = 10 * 20;
    let quotient = 14.59 / 18.9;
    let remainder = 50 % 30;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (400, 6.5, 1);

    // destructing seems to make a new value rather than passing the
    // stuff as reference at least for the scalar types
    let (mut x, mut y, mut z) = tup;

    // if you debug the code, you will see that the references are different
    let ref_x = &x;
    let ref_x_tuple = &tup.0;

    // still seems rust prints out true? is he checking the values rather than the refs? =O
    println!("The value of y = {:?}", ref_x == ref_x_tuple);

    let ref z = 42;

    x = 200;

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [10; 20];

    let first = a[0];
    let second = a[1];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is {}", index, element);

    another_function(x);
}

fn five() -> i32{
    5
}

fn plus_one(x: i32) -> i32{
    x + 1
}

fn another_function(x: i32){
    println!("The Value of x is {}", x);

    let y = {
        let x = 3;
        x +1
    };

    println!("Der Wert von y ist: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(10);

    println!("The value of x is: {}", x);
}

fn homework_celsius_to_fahrenheit(){
    //todo implement
}

fn homework_fibonacchi(depth: i32){
    // todo implement
}

fn homework_print_christmas_text(){
    // todo implement
}