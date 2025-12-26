fn main() {
    ex1();
    ex2();
    ex3();
}

fn ex1() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn ex2() {
    let _spaces = "     ";
    let _spaces = _spaces.len();

    // let mut _spaces = "     ";
    // _spaces = _spaces.len(); // Errors out -- cannot mutate variable's type (like you can in python)
}

fn ex3() {
    let heart_eyed_cat = '😻';
    println!("{} {}", heart_eyed_cat, heart_eyed_cat as u32);
    let c: u32 = 128571;
    println!("{:?}", char::from_u32(c));
}
