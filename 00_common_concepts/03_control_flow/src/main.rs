fn main() {
    r#loop();
    r#loop_label();
    r#while();
    r#while_for();
    r#for();
    r#for_range();
}

fn r#loop() {
    println!("-- BEGIN LOOP");

    let mut number = 3;

    let result = loop {

        if number < 10 {
            println!("Number less: {number}");
        } else {
            println!("Number more: {number}");
            break number *2;
        }

        number+= 1;
    };

    println!("Result loop: {result}");
    println!("-- END LOOP");
}

fn r#loop_label() {
    println!("-- BEGIN LOOP LABEL");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = count * 10;

        loop {
            println!("remaining = {remaining}");
            if remaining < -50 {
                break;
            }
            if count >= 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("Count = {count}");
    println!("-- END LOOP LABEL");
}

fn r#while() {
    println!("-- BEGIN WHILE");

    let mut number = 3;

    while number >= 0 {
        println!("Number = {number}");
        number -= 1;
    }

    println!("-- END WHILE");
}

fn r#while_for() {
    println!("-- BEGIN WHILE_FOR");

    let a:[i16; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value in the index {index} is: {}", a[index]);
        index += 1;
    }
    println!("-- END WHILE_FOR");
}

fn r#for() {
    println!("-- BEGIN FOR");

    let a = [1, 2, 3, 4, 5];

    for el in a {
        println!("Tje value is: {el}")
    }

    println!("-- END FOR");
}

fn r#for_range() {
    println!("-- BEGIN FOR RANGE");
    for number in (1..4).rev() {
        println!("{number}")
    }
    println!("-- END FOR RANGE");

}