use std::fs::File;
use super::user;
use std::io::Read;
pub fn run()
{
    //read user.txt to string
    let mut user_file = File::open("user.txt").expect("file not found");
    let mut user_string = String::new();
    user_file.read_to_string(&mut user_string).expect("something went wrong reading the file");

    let data = user_string.split("\n").collect::<Vec<&str>>();
    let user = data[0].trim();
    let pass = data[1].trim();
    println!("Enter username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim();
    println!("Enter password: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).expect("Failed to read line");
    let password = password.trim();
    if username == user && pass == password {
        let user: user::User = user::User::new(username.to_string(), password.to_string());
        super::mainMenu::menu(user);
    } else {
        println!("\nInvalid username or password");
    }
}