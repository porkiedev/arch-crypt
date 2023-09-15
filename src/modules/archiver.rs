use std::fs::OpenOptions;
use tar::{Builder, Archive};


pub fn pack(input_folder: String, output_tarball: String) {
    let output_file_options = OpenOptions::new()
    .write(true)
    .read(false)
    .create_new(true)
    .open(output_tarball);
    
    let output_file;
    match output_file_options {
        Ok(resp) => {
            output_file = resp;
        },
        Err(error) => {
            println!("ERROR: Couldn't create the output archive file:\n {error}");
            return;
        },
    };
    let mut tar_builder = Builder::new(output_file);
    
    match tar_builder.append_dir_all(input_folder.clone(), input_folder) {
        Ok(_) => {},
        Err(error) => {
            println!("ERROR: Failed to create the tar archive\n {error}");
            return;
        },
    };

    match tar_builder.into_inner() {
        Ok(_) => {},
        Err(error) => {
            println!("ERROR: Couldn't finalize the tar archive\n {error}");
            return;
        },
    }
}

pub fn unpack(input_tarball: String, output_folder: String) {
    let input_tarball_options = OpenOptions::new()
    .write(false)
    .read(true)
    .open(input_tarball);
    
    let input_tarball;
    match input_tarball_options {
        Ok(resp) => {
            input_tarball = resp;
        },
        Err(error) => {
            println!("ERROR: Couldn't open the input tarball:\n {error}");
            return;
        },
    };

    let mut tar_unpacker = Archive::new(input_tarball);

    match tar_unpacker.unpack(output_folder) {
        Ok(_) => {},
        Err(error) => {
            println!("ERROR: Failed to unpack the tar archive\n {error}");
            return;
        },
    };
}
