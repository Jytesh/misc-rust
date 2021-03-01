use rand::{thread_rng, Rng};
use std::io;

pub fn main() {
    let mut rng = thread_rng();
    println!("Enter your desired charset!");
    let mut charset = String::new();
    io::stdin()
        .read_line(&mut charset)
        .unwrap();
    let charset = String::from(charset.trim()).into_bytes();

    let mut size = String::new();
    println!("Enter your desired password length!");
    io::stdin()
        .read_line(&mut size)
        .unwrap();
    let size: u8 = match size.trim().parse() {
        Ok(x) => x,
        Err(_) => {
            eprintln!("Need an integer!");
            return;
        }
    };

    assert!(size > 0);
    assert!(size < 30);

    let password = 0..size;
    let password : Vec<char> = password.map(|_| -> char {
            let index = rng.gen_range(0..charset.len());
            charset[index] as char
        })
        .collect();
    println!("Your generated password: {:?}", password.into_iter().collect::<String>())

}
