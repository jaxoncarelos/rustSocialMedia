use super::user;
pub fn menu(user: user::User)
{
    println!("Hello {}", user.username);
    println!("1. View your profile");
    println!("2. Logout");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please type a number!");
    match choice
    {
        1 => println!("{} {}", user.username, user.password),
        2 => std::process::exit(69),
        _ => println!("Invalid choice")
    }
}