use std::io::{self, Write};

fn main() {
    let variable = "var1";
    println!("Vairable: {}", variable);

    let mut mut_variable = "var1";
    mut_variable = "var2";
    println!("Mut variable: {}", mut_variable);

    let num: i8 = 1;
    println!("I8 num: {}", num);

    let num: i64 = 65555;
    println!("Shadowing num: {}", num);

    println!("3+3={}", 3 + 3);
    println!("6-3={}", 6 - 3);
    println!("3*3={}", 3 * 3);
    println!("6/2={}", 6 / 2);
    println!("6.0/2.0={}", 6.0 / 2.0);
    println!("7%2={}", 7 % 2);

    let floatNum: f32 = 3.14;
    println!("{}", floatNum as i8);

    let raw_string = r"C:\Users\User\Ducuments\main.rs";
    println!("Raw string: {}", raw_string);

    print!("Enter your name: ");
    io::stdout().flush().expect("failed to flush stdout");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("failed to read line");
    println!("Hello, {}", name);

    print!("Enter your age: ");
    io::stdout().flush().expect("failed to flush stdout");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("failed to read line");

    let age: i64 = age.trim().parse().expect("failed to convert age");
    println!("Your age is {}", age);

    let mut num1 = String::new();
    let mut num2 = String::new();

    io::stdin().read_line(&mut num1).expect("Ошибка чтения");
    let num1: i32 = num1.trim().parse().expect("Ошибка преобразования");

    io::stdin().read_line(&mut num2).expect("Ошибка чтения");
    let num2: i32 = num2.trim().parse().expect("Ошибка преобразования");

    println!("Sum: {}", add(num1, num2));
    println!("Difference: {}", subtract(num1, num2));
    println!("Product: {}", multiply(num1, num2));
    println!("Quotient: {}", divide(num1, num2));
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y;
}

fn divide(x: i32, y: i32) -> i32 {
    return x / y;
}
