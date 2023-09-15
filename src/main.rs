// #![allow(unused)]
// #![allow(deprecated)]
use std::io::{BufRead, Write};

mod modules;
use modules::{encryption::{decrypt_file, encrypt_file}, archiver::{unpack, pack}, cli_args};


fn main() {
    let cli_arguments = cli_args::arguments().get_matches();
    
    match cli_arguments.subcommand() {
        Some(("pack", sub_matches)) => {
            // Parse arguments
            // Note: We can call .unwrap() on these arguments because they are required, and Clap ensures that they were provided
            let input_directory = sub_matches.get_one::<String>("INPUT_DIRECTORY").unwrap().to_owned();
            let output_file = sub_matches.get_one::<String>("OUTPUT_FILE").unwrap().to_owned();

            // Pack the directory (and its contents) into a tarball
            pack(input_directory, output_file);
        },
        Some(("unpack", sub_matches)) => {
            // Parse arguments
            // Note: We can call .unwrap() on these arguments because they are required, and Clap ensures that they were provided
            let input_file = sub_matches.get_one::<String>("INPUT_FILE").unwrap().to_owned();
            let output_directory = sub_matches.get_one::<String>("OUTPUT_DIRECTORY").unwrap().to_owned();

            // Unpack the contents of a tarball into a directory
            unpack(input_file, output_directory)
        },
        Some(("encrypt", sub_matches)) => {
            // Parse arguments
            // Note: We can call .unwrap() on these arguments because they are required, and Clap ensures that they were provided
            let input_file = sub_matches.get_one::<String>("INPUT_FILE").unwrap().to_owned();
            let output_file = sub_matches.get_one::<String>("OUTPUT_FILE").unwrap().to_owned();

            // Prompt user for a password and hash it into encryption key
            let plaintext_password;
            match prompt_user_for_password(true) {
                Ok(resp) => {
                    plaintext_password = resp;
                },
                Err(_) => {
                    return;
                }
            };

            // Encrypt the file
            match encrypt_file(input_file, output_file, plaintext_password) {
                Ok(_) => {},
                Err(error) => {
                    println!("ERROR: Failed to encrypt the file:\n {error}");
                }
            };
        },
        Some(("decrypt", sub_matches)) => {
            // Parse arguments
            // Note: We can call .unwrap() on these arguments because they are required, and Clap ensures that they were provided
            let input_file = sub_matches.get_one::<String>("INPUT_FILE").unwrap().to_owned();
            let output_file = sub_matches.get_one::<String>("OUTPUT_FILE").unwrap().to_owned();

            // Prompt user for a password and hash it into encryption key
            let plaintext_password;
            match prompt_user_for_password(false) {
                Ok(resp) => {
                    plaintext_password = resp;
                },
                Err(_) => {
                    return;
                }
            };

            // Decrypt file
            match decrypt_file(input_file, output_file, plaintext_password) {
                Ok(_) => {},
                Err(error) => {
                    println!("ERROR: Failed to decrypt the file:\n {error}");
                }
            };
        },
        _ => {}
    };
}

fn prompt_user_for_password(password_confirmation: bool) -> Result<String, ()> {

    // Initialize variables
    let mut password = String::new();

    // Prompt the user for a passsword
    print!("Input a password\n> ");
    std::io::stdout().flush().unwrap();
    let mut stdin = std::io::stdin().lock();
    stdin.read_line(&mut password).unwrap();

    // Remove special characters from the password
    let password = password.trim();

    if password_confirmation {
        // Initialize annother variable for the confirmation password
        let mut password_again = String::new();

        // Prompt the user to confirm their password
        print!("Confirm your password\n> ");
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut password_again).unwrap();

        // Remove special characters from the confirmation password
        let password_again = password_again.trim();

        // Make sure the passwords match
        if password != password_again {
            println!("ERROR: Passwords don't match");
            return Err(());
        };
    }

    // Return the password
    return Ok(password.to_string());
}
