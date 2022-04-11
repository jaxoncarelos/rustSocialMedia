use super::*;
use std::fs::File;
use std::io::Write;
pub fn run()
{
    println!("Enter username: ");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();

    println!("Enter password: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();

    let mut w = File::create("user.txt").unwrap();
    writeln!(&mut w, " {} {}", username, password).unwrap();
    let user = super::user::User::new(username.trim().to_string(), password.trim().to_string());
    super::mainMenu::menu(user);
}