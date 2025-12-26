fn main() {
    ex1_if_one_liner(true);
    ex2_return_value_from_loop();
    ex3_naming_loops_and_breaking_out_by_label();
    ex4_while_loops_syntax();
    ex5_foreach_loop();
    ex6_reversed_for_loop();
}

fn ex1_if_one_liner(condition: bool) {
    let alpha = if condition { 6 } else { 7 };
    println!("Condition alpha: {}", alpha);
}

fn ex2_return_value_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn ex3_naming_loops_and_breaking_out_by_label() {
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
}

fn ex4_while_loops_syntax() {
    let mut num = 3;
    while num != 0 {
        println!("WHILE: {num}");
        num -= 1;
    }
}

fn ex5_foreach_loop() {
    let a = [8, 6, 7, 5, 3, 0, 9];
    for e in a {
        println!("Foreach: {}", e);
    }
}

fn ex6_reversed_for_loop() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
