use std::io;

fn sqrt(x: i32) -> i32 {
    let mut s: i32 = 0;
    while s * s < x {
        s += 1;
    }
    return s;
}

fn is_prime(x: i32) -> bool {
    let s: i32 = sqrt(x);
    for i in 2..s {
        if x % i == 0 {
            return false;
        }
    }
    return x >= 2;
}

fn main() {
    println!("Hello, world!");
    let mut x: i32 = 1;
    x = 9;
    println!("{}", is_prime(x));

    let mut s: String = read_string();
    println!("Received {}", s);

    let mut n: i32 = read_i32();
    println!("Received {}", n * 2);
}

fn read_string() -> String {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).expect("Invalid input");
    return s;
}

fn read_i32() -> i32 {
    let mut t: String = read_string();
    let mut n: i32 = t.trim().parse::<i32>().expect("Invalid input");
    return n;
}