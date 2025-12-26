fn main() {
    for i in 0..93 {
        // i64 max reached at f(93)
        println!("{}\t{}", i, fib_fn(i));
    }
}

// Very slow
fn fib_rec(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if n < 0 {
        return pow(-1, -n + 1) * fib_rec(-n);
    }

    return fib_rec(n - 1) + fib_rec(n - 2);
}

fn pow(a: i32, b: i32) -> i32 {
    let mut o: i32 = 1;
    for _i in 0..b {
        o *= a;
    }
    return o;
}

fn fpow(a: f64, b: i32) -> f64 {
    let mut o: f64 = 1.0;
    for _i in 0..b {
        o *= a;
    }
    return o;
}

// Uses closed-form expression w/ Golden Ratio:
// cr https://en.wikipedia.org/wiki/Fibonacci_sequence#Relation_to_the_golden_ratio
fn fib_fn(n: i32) -> i64 {
    let phi: f64 = 1.61803398875;
    let psi: f64 = -0.61803398875;
    return ((fpow(phi, n) - fpow(psi, n)) / (phi - psi)) as i64;
}
