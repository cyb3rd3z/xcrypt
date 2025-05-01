use rfd::FileDialog;
use std::fs::{self};
use std::io;
use std::io::Write;
use std::process;

fn encrypt(contents: Vec<u8>) -> Vec<u8> {
    println!("Enter key: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to get key");
    let key: u8 = user_input.trim().parse().expect("failed converting key.");
    contents.iter().map(|byte| byte.wrapping_add(key)).collect()
}

fn decrypt(contents: Vec<u8>) -> Vec<u8> {
    println!("Enter key: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to get key");
    let key: u8 = user_input.trim().parse().expect("failed converting key");
    contents.iter().map(|byte| byte.wrapping_sub(key)).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("\n\t\tXcrypt\n");
        println!("  1) encrypt file");
        println!("  2) decrypt file");
        println!("  3) quit");

        println!("\nenter choice below: ");
        let mut user = String::new();
        io::stdin()
            .read_line(&mut user)
            .expect("failed to get user input.");
        let input = user.as_str().trim();

        match input {
            "1" => {
                let f = FileDialog::new().pick_file().unwrap();
                let trimmed_path = f.display().to_string();
                let path = trimmed_path.as_str().trim();
                match fs::read(path) {
                    Ok(contents) => {
                        let encrypted_text = encrypt(contents);
                        let mut new_file = fs::OpenOptions::new().write(true).open(path).unwrap();
                        if let Err(e) = new_file.write_all(&encrypted_text) {
                            println!("Error writting to file: {e}")
                        }
                    }
                    Err(e) => {
                        println!("could not open file: {e}");
                    }
                }
            }

            "2" => {
                let f = FileDialog::new().pick_file().unwrap();
                let trimmed_path = f.display().to_string();
                let path = trimmed_path.as_str().trim();
                match fs::read(path) {
                    Ok(contents) => {
                        let encrypted_text = decrypt(contents);
                        let mut new_file = fs::OpenOptions::new().write(true).open(path).unwrap();
                        if let Err(e) = new_file.write_all(&encrypted_text) {
                            println!("Error writting to file: {e}")
                        }
                    }
                    Err(e) => {
                        println!("could not open file: {e}");
                    }
                }
            }

            "3" => {
                process::exit(1);
            }

            _ => {
                println!("invalid input.")
            }
        }
    }
}
