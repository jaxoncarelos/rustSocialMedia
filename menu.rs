use std::io;
mod signin;
mod signup;
mod mainMenu;
mod user;
pub fn run()
{
    println!("Hello user.");
    println!("Please select an option:");
    println!("1. Sign in");
    println!("2. Sign up");
    println!("3. Exit");
    let stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_line(&mut buff).expect("Failed to read line");
    println!("You selected {}", buff);
    // convert buff to i32
    let choice: i8 = buff.trim().parse().expect("Please enter a number");
    match choice{
    
        1 => {
            signin::run();
        },
        2 => {
            signup::run();
        },
        3 => {
            std::process::exit(0);
        },
        _ => {
            println!("{} is Invalid input", buff);
        }
    }
}