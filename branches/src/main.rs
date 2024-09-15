
fn main() {
    normal_if(3);
    normal_if(7);
    normal_if(12);

    declare_with_condition(true);
    declare_with_condition(false);

    repeat_with_loop();
    returning_value_with_loop();
    labeled_loop();
    conditional_with_while();
    looping_with_collections();
}

fn normal_if(x: i32) {
    if x < 5 {
        println!("The number is less than 5");
    } else if x > 10 {
        println!("The number is higher than 10");
    } else {
        println!("The number is between 5 and 10");
    }
}

fn declare_with_condition(condition: bool) {
    let number = if condition { 5 } else { 6 };
    println!("The condition is {condition} so the number is {number}");
}

fn repeat_with_loop() {
    let mut times = 0;

    loop {
        if times == 5 { break };

        println!("Loop");

        times += 1;
    }
}


fn returning_value_with_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {result}");
}

fn labeled_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    };

    println!("ENd count = {count}")
}

fn conditional_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!")
}

fn looping_with_collections() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("The value using for is: {element}")
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}