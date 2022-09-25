use std::io;

fn main() {
    println!("\nThis program generates the nth Fibonacci Number");

    loop {
        println!("\nEnter the value to generate a nth Fibonacci Number");
        let mut user_value = String::new();
        io::stdin()
            .read_line(&mut user_value)
            .expect("Failed to read line.");
        let user_value: f64 = user_value.trim().parse().expect("Please type a number");

        let five: f64 = 5.0;
        let square_root_five: f64 = five.sqrt();
        let value: f64 = (1.0 + square_root_five)/2.0;
        let fibonacci_number: f64 = (value.powf(user_value) / square_root_five).round();

        println!("The {user_value}th Fibonacci Number is {fibonacci_number}\n");

        println!("Do you want to generate another nth Fibonacci Number: Y for yes or N for no");
        let mut user_continue = String::new();
        io::stdin()
            .read_line(&mut user_continue)
            .expect("Failed to read line.");
        let user_continue = user_continue.trim();

        if user_continue == "N" {
            break println!("\nThanks for using the app. Good Bye!!\n");
        }
    }
}
