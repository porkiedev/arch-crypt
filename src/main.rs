// #![allow(unused)]
// #![allow(deprecated)]
use std::io::{BufRead, Write};
use log::{info, error};
mod modules;
use modules::{encryption::{decrypt_file, encrypt_file}, archiver::{unpack, pack}, cli_args};


fn main() {
    // Initialize logger
    env_logger::builder().format_timestamp(None).filter_level(log::LevelFilter::Trace).init();
    info!("Starting");

    // Handle CLI arguments
    let cli_arguments = cli_args::arguments().get_matches();

    if let Some(("pack", sub_matches)) = cli_arguments.subcommand() {
        info!("User requested 'pack'");

        // Parse arguments
        // Note: We can call .unwrap() on these arguments because they are required, and Clap ensures that they were provided
        let input_directory = sub_matches.get_one::<String>("INPUT_DIRECTORY").unwrap().to_owned();
        let output_file = sub_matches.get_one::<String>("OUTPUT_FILE").unwrap().to_owned();

        // Pack the directory (and its contents) into a tarball
        match pack(input_directory, output_file) {
            Ok(_resp) => {},
            Err(_error) => {
                error!("Failed to pack tarball");
                return;
            },
        };
    }

    if let Some(("unpack", sub_matches)) = cli_arguments.subcommand() {
        info!("User requested 'unpack'");

        // Parse arguments
        // Note: We can call .unwrap() on these arguments because they are required, and Clap ensures that they were provided
        let input_file = sub_matches.get_one::<String>("INPUT_FILE").unwrap().to_owned();
        let output_directory = sub_matches.get_one::<String>("OUTPUT_DIRECTORY").unwrap().to_owned();

        // Unpack the contents of a tarball into a directory
        match unpack(input_file, output_directory) {
            Ok(_resp) => {},
            Err(_error) => {
                error!("Failed to unpack tarball");
                return;
            },
        };
    }

    if let Some(("encrypt", sub_matches)) = cli_arguments.subcommand() {
        info!("User requested 'encrypt'");

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
    }

    if let Some(("decrypt", sub_matches)) = cli_arguments.subcommand() {
        info!("User requested 'decrypt'");

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
        match decrypt_file(input_file.clone(), output_file, plaintext_password) {
            Ok(_) => {},
            Err(error) => {
                error!("Failed to decrypt the file '{input_file}':\n {error}");
            }
        };
    }

}

fn prompt_user_for_password(should_confirm_password: bool) -> Result<String, ()> {

    // Initialize variables
    let mut password = String::new();

    // Prompt the user for a passsword
    print!("Input a password\n> ");
    match std::io::stdout().flush() {
        Ok(_resp) => {},
        Err(error) => {
            error!("Failed to flush the output (stdout) stream:\n {error}");
            return Err(());
        }
    };

    let mut stdin = std::io::stdin().lock();
    match stdin.read_line(&mut password) {
        Ok(_resp) => {},
        Err(error) => {
            error!("Failed to read input password:\n {error}");
            return Err(());
        }
    };

    // Remove special characters from the password
    let password = password.trim();

    if should_confirm_password {
        // Initialize annother variable for the confirmation password
        let mut password_again = String::new();

        // Prompt the user to confirm their password
        print!("Confirm your password\n> ");
        match std::io::stdout().flush() {
            Ok(_resp) => {},
            Err(error) => {
                error!("Failed to flush the output (stdout) stream:\n {error}");
                return Err(());
            }
        };
        match stdin.read_line(&mut password_again) {
            Ok(_resp) => {},
            Err(error) => {
                error!("Failed to read input password:\n {error}");
                return Err(());
            }
        };

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
