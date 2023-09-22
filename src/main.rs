/*
Copyright (C) 2023 Elijah Fry

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses/gpl-3.0.html.
*/


// #![allow(unused)]
// #![allow(deprecated)]
use std::io::{BufRead, Write};
use log::{info, error};
mod modules;
use modules::{encryption::{decrypt_file, encrypt_file}, archiver::{unpack, pack}, cli_args};
use zeroize::Zeroize;


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
        let plaintext_password = match prompt_user_for_password(true) {
            Ok(resp) => {
                resp
            },
            Err(_error) => {
                return;
            }
        };

        // Encrypt the file
        let _ =  encrypt_file(input_file, output_file, plaintext_password);
    }

    if let Some(("decrypt", sub_matches)) = cli_arguments.subcommand() {
        info!("User requested 'decrypt'");

        // Parse arguments
        // Note: We can call .unwrap() on these arguments because they are required, and Clap ensures that they were provided
        let input_file = sub_matches.get_one::<String>("INPUT_FILE").unwrap().to_owned();
        let output_file = sub_matches.get_one::<String>("OUTPUT_FILE").unwrap().to_owned();

        // Prompt user for a password and hash it into encryption key
        let plaintext_password = match prompt_user_for_password(false) {
            Ok(resp) => {
                resp
            },
            Err(_error) => {
                return;
            }
        };

        // Decrypt file
        let _ = decrypt_file(input_file.clone(), output_file, plaintext_password);
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
    let password = password.trim().to_string();

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
        let mut password_again = password_again.trim().to_string();

        // Make sure the passwords match
        if password != password_again {
            error!("Passwords don't match");
            return Err(());
        };

        // Flush the memory that holds our confirmation password with 0s for some extra security
        password_again.zeroize();
    }

    // Return the password
    Ok(password.to_string())
}
