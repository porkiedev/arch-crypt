use std::{fs::{OpenOptions, remove_file, remove_dir_all}, path::Path};
use log::error;
use tar::{Builder, Archive};


pub fn pack<T: AsRef<Path>>(input_folder: T, output_tarball: T) -> Result<(), ()> {
    // Try to open an instance of the output_tarball
    let output_file_options = OpenOptions::new()
    .write(true)
    .append(false)
    .read(false)
    .create_new(true)
    .open(&output_tarball);
    
    // Ensure that we created the output_tarball safely
    let output_file = match output_file_options {
        Ok(resp) => {
            resp
        },
        Err(error) => {
            error!("Failed to create the output archive file:\n {error}");
            return Err(());
        }
    };

    // Create an instance of the archive builder
    let mut tar_builder = Builder::new(output_file);
    
    // Use the archive builder to clone the input_folder into the output_tarball
    match tar_builder.append_dir_all(&input_folder, &input_folder) {
        Ok(_) => {},
        Err(error) => {
            // let _ = delete_file(output_tarball);
            error!("Failed to create the tar archive:\n {error}");
            return Err(());
        }
    };

    // Finalize the output_tarball, appending the termination bytes to the end of the output_tarball
    match tar_builder.into_inner() {
        Ok(_resp) => {},
        Err(error) => {
            // let _ = delete_file(output_tarball);
            error!("Failed to finish writing to the tar archive:\n {error}");
            return Err(());
        }
    }

    // Return the success!
    Ok(())
}

// Unpacks a tarball's contents into the specified output_folder
pub fn unpack<T: AsRef<Path>>(input_tarball: T, output_folder: T) -> Result<(), ()> {
    // Try to open the input_tarball
    let input_tarball_options = OpenOptions::new()
    .write(false)
    .read(true)
    .open(input_tarball);
    
    // Make sure that we safely opened the input_tarball
    let input_tarball = match input_tarball_options {
        Ok(resp) => {
            resp
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
            // let _ = delete_directory_recursively(output_folder);
            error!("Failed to unpack the input tarball:\n {error}");
            return Err(());
        },
    };

    // Return our success!
    Ok(())
}

// Used to delete a file. Useful if we failed to create a tarball and want to clean up the mess
#[allow(dead_code)]
pub fn delete_file<T: AsRef<Path>>(input_file: T) -> Result<(), ()> {
    // Try to delete the input_file
    match remove_file(&input_file) {
        Ok(_resp) => {
            Ok(())
        },
        Err(error) => {
            error!("Failed to delete file at '{:?}':\n {error}", input_file.as_ref());
            Err(())
        },
    }
}

// Used to delete a directory. Useful if we failed while unpacking a tarball and want to clean up the mess
#[allow(dead_code)]
pub fn delete_directory_recursively<T: AsRef<Path>>(input_directory: T) -> Result<(), ()> {
    // Try to recursively delete the input_directory
    match remove_dir_all(&input_directory) {
        Ok(_resp) => {
            Ok(())
        },
        Err(error) => {
            error!("Failed to delete directory at '{:?}':\n {error}", input_directory.as_ref());
            Err(())
        },
    }
}
