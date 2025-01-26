use clap::{Arg, Command};
use rand::Rng;
use std::iter;

fn generate_password(length: usize, include_uppercase: bool, include_numbers: bool, include_special: bool) -> String {
    let mut charset = "abcdefghijklmnopqrstuvwxyz".to_string();

    if include_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    if include_numbers {
        charset.push_str("0123456789");
    }

    if include_special {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?/`~");
    }

    let password: String = iter::repeat_with(|| {
        let idx = rand::thread_rng().gen_range(0..charset.len());
        charset.chars().nth(idx).unwrap()
    })
    .take(length)
    .collect();

    password
}

fn main() {
    
    let matches = Command::new("Password Generator")
    .version("1.0")
    .author("Jeykison <jeykison2000@gmail.com>")
    .about("A simple random password generator")
    .arg(
        Arg::new("length")
            .short('l')
            .long("length")
            .value_name("LENGTH")
            .help("Specifies the length of the password")
            .default_value("12")
            .value_parser(clap::value_parser!(usize)),
    )
    .arg(
        Arg::new("uppercase")
            .short('u')
            .long("uppercase")
            .help("Include uppercase letters")
            .num_args(0),
    )
    .arg(
        Arg::new("numbers")
            .short('n')
            .long("numbers")
            .help("Include numbers")
            .num_args(0),
    )
    .arg(
        Arg::new("special")
            .short('s')
            .long("special")
            .help("Include special characters")
            .num_args(0),
    )
    .get_matches();


    let length: usize = *matches.get_one::<usize>("length").unwrap_or(&12);
    let include_uppercase = matches.contains_id("uppercase");
    let include_numbers = matches.contains_id("numbers");
    let include_special = matches.contains_id("special");

    let password = generate_password(length, include_uppercase, include_numbers, include_special);

    println!("Generated Password: {}", password);
}