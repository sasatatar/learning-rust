fn main() {
    let number = 6;

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // if is an expression so the result can be assigned to a variable
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    if loops() {
        while_loop();
    };

    println!();

    range_example();
}

fn loops() -> bool {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    result == 20
}

// upper range limit is exclusive!
fn range_example() {
    for number in (1..11).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}