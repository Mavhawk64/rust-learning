fn main() {
    print_labeled_measurement(5, 'h');
    ex1();
    ex2();
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn ex1() {
    let y = {
        let x = 3;
        x + 1
    }; // this curly is an expression

    {
        let x = 3;
    } // this curly is a syntactic scope

    println!("The value of y is: {y}");
}

fn ex2() {
    let x = add_two(67);

    println!("The value of x is: {x}");
}

fn add_two(x: i128) -> i128 {
    x + 2
}
