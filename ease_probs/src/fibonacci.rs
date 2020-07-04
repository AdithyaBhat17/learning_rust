use std::io;

fn main() {
    println!("Enter a value for n");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Enter a valid input");

    let n: i32 = n.trim().parse().expect("Enter a valid number");
    let result: i64 = nth_fibonacci(n);

    println!("The {}th term in the fibonacci series is {}", n, result);
}

fn nth_fibonacci(number: i32) -> i64 {
    let mut num1 = 0;
    let mut num2 = 1;
    let mut nth_term: i64 = 0;
    let mut counter = 0;
    while counter <= number {
        nth_term = num1 + num2;
        num1 = num2;
        num2 = nth_term;
        counter += 1;
    }
    return nth_term;
}
