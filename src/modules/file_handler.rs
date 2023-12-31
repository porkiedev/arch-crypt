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


use std::fs::{File, OpenOptions, Metadata};
use std::io::{Read, Write};
use log::error;


pub struct FileReaderWriter {
    input_file: File,
    pub input_file_metadata: Metadata,
    output_file: File
}

impl FileReaderWriter {
    pub fn new(input_file_name: &str, output_file_name: &str) -> Result<Self, ()> {
        let input_file = OpenOptions::new()
        .write(false)
        .read(true)
        .open(input_file_name);

        let output_file = OpenOptions::new()
        .write(true)
        .read(false)
        .create(true)
        .open(output_file_name);

        let input_file = match input_file {
            Ok(input_file) => input_file,
            Err(error) => {
                error!("Couldn't open the input file '{input_file_name}':\n {error}");
                return Err(());
            }
        };

        let output_file = match output_file {
            Ok(output_file) => output_file,
            Err(error) => {
                error!("Couldn't open the output file '{output_file_name}':\n {error}");
                return Err(());
            }
        };

        let input_file_metadata = match input_file.metadata() {
            Ok(input_file_metadata) => input_file_metadata,
            Err(error) => {
                error!("Couldn't get metadata from input file:\n {error}");
                return Err(());
            }
        }; 

        // This is currently unneeded, but consider implementing a BufReader in the future if performance becomes a big concern
        // let mut bufrdr = std::io::BufReader::new(file);

        Ok(Self {
            input_file,
            input_file_metadata,
            output_file
        })

    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, ()> {
        match self.input_file.read(buffer) {
            Ok(num_bytes_read) => {
                Ok(num_bytes_read)
            },
            Err(error) => {
                error!("Failed to read bytes from input file:\n {error}");
                Err(())
            }
        }
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<usize, ()> {
        match self.output_file.write(buffer) {
            Ok(num_bytes_written) => {
                Ok(num_bytes_written)
            },
            Err(error) => {
                error!("Failed to write bytes to output file:\n {error}");
                Err(())
            }
        }
    }
}

