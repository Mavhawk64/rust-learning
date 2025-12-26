fn main() {
    ex1();
    let run_tups = false;
    if run_tups {
        ex2();
    }
    ex3();
}

fn ex1() {
    let d = 123_198_222; // use _ to separate thousands for Decimal (base-10)
    let h = 0xff; // hex
    let o = 0o77; // octal
    let b = 0b1111_0000; // binary
    let y: u8 = b'a'; // byte (only u8)

    println!("Check out all these numbers! {} {} {} {} {}", d, h, o, b, y);
}

fn ex2() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    println!("The value of _y is {_y}");

    println!("The second value of tup is {}", tup.1);

    let mut tup2: (f32, f32) = (-1.0, 0.0);
    println!(
        "tup2 is mutable! Check out it's values as it changes: ({}, {})",
        tup2.0, tup2.1
    );
    for i in 0..21 {
        tup2.1 += i as f32 / 100.0;
        if i % 2 == 0 {
            tup2.0 -= i as f32 / tup2.0;
        }
        println!("tup2 looks like this now: ({}, {})", tup2.0, tup2.1);
    }
    let mut tup3: (u128, u128) = (9, 9);
    loop {
        println!("tup3: ({}, {})", tup3.0, tup3.1);
        if tup3.1 == 1 {
            break;
        }
        tup3.1 = collatz(tup3.1);
    }
}

fn collatz(n: u128) -> u128 {
    if n % 2 == 0 {
        return n / 2;
    }
    return 3 * n + 1;
}

fn ex3() {
    let _a = [1, 2, 3, 4, 5];
    let _b = [0; 100]; // 100 x 0's
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
