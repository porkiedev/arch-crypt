use std::fs::{OpenOptions, remove_file, remove_dir_all};
use log::error;
use tar::{Builder, Archive};


pub fn pack(input_folder: String, output_tarball: String) -> Result<(), ()> {
    // Try to open an instance of the output_tarball
    let output_file_options = OpenOptions::new()
    .write(true)
    .append(false)
    .read(false)
    .create_new(true)
    .open(&output_tarball);
    
    // Ensure that we created the output_tarball safely
    let output_file;
    match output_file_options {
        Ok(resp) => {
            output_file = resp;
        },
        Err(error) => {
            error!("Failed to create the output archive file:\n {error}");
            return Err(());
        }
    };

    // Create an instance of the archive builder
    let mut tar_builder = Builder::new(output_file);
    
    // Use the archive builder to clone the input_folder into the output_tarball
    match tar_builder.append_dir_all(input_folder.clone(), input_folder) {
        Ok(_) => {},
        Err(error) => {
            delete_file(output_tarball).unwrap();
            error!("Failed to create the tar archive:\n {error}");
            return Err(());
        }
    };

    // Finalize the output_tarball, appending the termination bytes to the end of the output_tarball
    match tar_builder.into_inner() {
        Ok(_resp) => {},
        Err(error) => {
            delete_file(output_tarball).unwrap();
            error!("Failed to finish writing to the tar archive:\n {error}");
            return Err(());
        }
    }

    // Return the success!
    return Ok(());
}

pub fn unpack(input_tarball: String, output_folder: String) -> Result<(), ()> {
    // Try to open the input_tarball
    let input_tarball_options = OpenOptions::new()
    .write(false)
    .read(true)
    .open(input_tarball);
    
    // Make sure that we safely opened the input_tarball
    let input_tarball;
    match input_tarball_options {
        Ok(resp) => {
            input_tarball = resp;
        },
        Err(error) => {
            error!("Failed to open the input tarball:\n {error}");
            return Err(());
        }
    };

    // Create an instance of the archiver
    let mut tar_unpacker = Archive::new(input_tarball);

    // Try to unpack the input_tarball into the output_folder
    match tar_unpacker.unpack(&output_folder) {
        Ok(_resp) => {},
        Err(error) => {
            delete_directory_recursively(output_folder).unwrap();
            error!("Failed to unpack the input tarball:\n {error}");
            return Err(());
        },
    };

    // Return our success!
    return Ok(());
}

pub fn delete_file(input_file: String) -> Result<(), ()> {
    match remove_file(input_file.clone()) {
        Ok(_resp) => {
            return Ok(());
        },
        Err(error) => {
            error!("Failed to delete file at '{input_file}':\n {error}");
            return Err(());
        },
    };
}

pub fn delete_directory_recursively(input_directory: String) -> Result<(), ()> {
    match remove_dir_all(input_directory.clone()) {
        Ok(_resp) => {
            return Ok(());
        },
        Err(error) => {
            error!("Failed to delete directory at '{input_directory}':\n {error}");
            return Err(());
        },
    };
}
