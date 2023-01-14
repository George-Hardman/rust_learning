fn main() {
    let number = 7;
    if number != 0 {
        println!("Not zero");
    }

    println!("\n---- Handling multiple conditions ----");


    let number = 8;
    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("not divisible by 4, 3, 2");
    }


    println!("\n---- Conditional assignment----");

    let condition = true;
    let number = if condition {5} else {6};
    println!("The val of number is {number}");


    println!("\n---- loops----");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; 
        }
    };
    println!("The result is {result}");


    println!("\n---- Loops with labels----");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
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
    }

    println!("End count = {count}");


    println!("---- While loops----");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF");

    println!("/n---- For loops----");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is {}", element);
    } 

    println!("----ranges----");

    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF");

}
