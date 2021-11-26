fn main() {
    let number = 7;

    if number != 0 {
        println!("Number is not null");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of the number is {}", number);

    two_loops_control();
    loop_return_result();
    while_loop();
    while_loop_over_array();
    for_loop_over_array();
}

fn two_loops_control(){
    let mut count = 0;

    'counting_up: loop{
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);

            if remaining == 9{
                break;
            }

            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

fn loop_return_result(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };

    println!("The result is {}",result);
}

fn while_loop(){
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("TO THE MOON!");
}

fn while_loop_over_array(){
    let a = [1,2,3];
    let a = a.map(|v| v +1);
    println!("{:?}", a);
    let mut index = 0;
    while index < a.len() {
        println!("The value of {}", a[index]);

        index += 1;
    }
}

fn for_loop_over_array(){
    let a = [1,2,3,4,5];
    for element in a {
        println!("Current value: {}", element);
    }
}

fn for_loop_over_array_initializer(){
    let start = 10;
    let end = 100;

    for number in (start..end).rev(){
        println!("{}", number);
    }

    println!("TAKE OFF!!");
}