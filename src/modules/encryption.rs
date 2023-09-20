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


use argon2::{Argon2, Error, password_hash::rand_core::RngCore};
use chacha20::ChaCha20;
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305,
    ChaChaPoly1305,
    consts::U12
};
use log::error;
use crate::modules::file_handler::FileReaderWriter;


// The BLOCK_SIZE is the size of bytes we encrypt at a time
// Note that when encrypting a block of bytes, a 16byte 'checksum' of sorts will be appended to the end (BLOCK_SIZE + 16)
const BLOCK_SIZE: usize = 32768;


/// Hashes a password using the Argon2id algorithm
/// 
/// # Arguments
/// * `plaintext_password` - The password that you want hashed
/// * `input_salt` - Optional, Allows you to specify a salt to use when hashing the password
/// 
/// # Notes
/// Choosing an input_salt is useful when trying to recreate an encryption key used to encrypt a file
/// 
/// # Returns
/// A tuple containing the hashed password and the salt used.
fn hash_password(plaintext_password: String, input_salt: Option<[u8; 32]>) -> Result<([u8; 32], [u8; 32]), Error> {
    // Variable declarations
    let mut password_hash = [0u8; 32];
    let mut salt = [0u8; 32];

    // If we were provided with a pre-determined salt, use that, otherwise generate a random salt
    match input_salt {
        Some(resp) => {
            salt = resp;
        },
        None => {
            OsRng.fill_bytes(&mut salt);
        },
    }

    // Compute 256-bit hash from plaintext password
    match Argon2::default().hash_password_into(plaintext_password.as_bytes(), &salt, &mut password_hash) {
        Ok(_) => {},
        Err(error) => {
            return Err(error);
        },
    };

    // Return the hash and the salt we used
    let encryption_key = (password_hash, salt);
    
    Ok(encryption_key)
}

/// Encrypts a a file using a plaintext password
/// 
/// # Arguments
/// * `input_file` - The location of the file you want to encrypt
/// * `output_file` - The location of the output file
/// * `plaintext_password` - The password that you want use (It will be hashed using the Argon2id algorithm)
pub fn encrypt_file(input_file: String, output_file: String, plaintext_password: String) -> Result<(), ()> {
    // Try to initialize I/O file writer
    let mut file_rw;
    match FileReaderWriter::new(&input_file, &output_file) {
        Ok(resp) => file_rw = resp,
        Err(_error) => {
            return Err(());
        }
    };

    // Hash plaintext_password into a 256bit key
    let (encryption_key, salt);
    match hash_password(plaintext_password, None) {
        Ok(resp) => {
            (encryption_key, salt) = resp;
        },
        Err(_error) => {
            return Err(());
        }
    };

    // Initialize cryptor
    let cryptor = match Cryptor::new(encryption_key, None) {
        Ok(resp) => {
            resp
        },
        Err(_error) => {
            return Err(());
        },
    };

    // Write salt and nonce to the start of the output file
    match file_rw.write(&salt) {
        Ok(_resp) => {},
        Err(_error) => {
            return Err(());
        }
    };
    match file_rw.write(&cryptor.nonce) {
        Ok(_resp) => {},
        Err(_error) => {
            return Err(());
        }
    };

    // Define a few variables
    let total_file_size_bytes = file_rw.input_file_metadata.len() as usize;
    let mut total_num_bytes_read: usize = 0;
    let mut read_file_buffer = [0u8; BLOCK_SIZE];

    // Iterate through the input file in (BLOCK_SIZE) chunks and encrypt the bytes
    while total_num_bytes_read < total_file_size_bytes {
        // Try to read bytes from the input file
        let num_bytes_read = match file_rw.read(&mut read_file_buffer) {
            Ok(resp) => resp,
            Err(_error) => {
                return Err(());
            }
        };
        total_num_bytes_read += num_bytes_read;

        // Encrypt the bytes and try to write them to the output file
        let encrypted_bytes = match cryptor.encrypt_bytes(&read_file_buffer[..num_bytes_read]) {
            Ok(resp) => {
                resp
            },
            Err(_error) => {
                return Err(());
            },
        };
        
        // Try to write encrypted bytes to the output file
        match file_rw.write(encrypted_bytes.as_ref()) {
            Ok(_resp) => {},
            Err(_error) => {
                return Err(());
            }
        };
    }
    Ok(())
}

/// Decrypts a a file using a plaintext password
/// 
/// # Arguments
/// * `input_file` - The location of the encrypted file
/// * `output_file` - The location where you want the output file
/// * `plaintext_password` - The password you used to encrypt the file
/// 
/// # Notes
/// The plaintext_password will be hashed using the Argon2id algorithm and the salt that was stored in the file during the initial encryption process
pub fn decrypt_file(input_file: String, output_file: String, plaintext_password: String) -> Result<(), ()> {
    // Try to initialize input and output file writer
    let mut file_rw;
    match FileReaderWriter::new(&input_file, &output_file) {
        Ok(resp) => file_rw = resp,
        Err(_error) => {
            return Err(());
        }
    };

    // Read salt and nonce from the start of the input file
    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 12];
    match file_rw.read(&mut salt) {
        Ok(_resp) => {},
        Err(_error) => {
            return Err(());
        }
    };
    match file_rw.read(&mut nonce) {
        Ok(_resp) => {},
        Err(_error) => {
            return Err(());
        }
    };

    // Hash plaintext_password into a 256bit key
    let encryption_key;
    match hash_password(plaintext_password, Some(salt)) {
        Ok(resp) => {
            (encryption_key, _) = resp;
        },
        Err(_error) => {
            return Err(());
        }
    };

    // Initialize the cryptor
    let cryptor = match Cryptor::new(encryption_key, Some(nonce)) {
        Ok(resp) => {
            resp
        },
        Err(_error) => {
            return Err(());
        },
    };

    // Define a few variables
    let total_file_size_bytes = file_rw.input_file_metadata.len() as usize;
    let mut total_num_bytes_read: usize = 44;
    let mut read_file_buffer = [0u8; BLOCK_SIZE + 16]; // We must read 16 extra bytes for the Poly1305 checksum

    // Iterate through the input file in (BLOCK_SIZE) chunks and encrypt the bytes
    while total_num_bytes_read < total_file_size_bytes {
        // Try to read bytes from the input file
        let num_bytes_read = match file_rw.read(&mut read_file_buffer) {
            Ok(resp) => resp,
            Err(_error) => {
                return Err(());
            }
        };
        total_num_bytes_read += num_bytes_read;

        // Encrypt the bytes and try to write them to the output file
        let decrypted_bytes = match cryptor.decrypt_bytes(&read_file_buffer[..num_bytes_read]) {
            Ok(resp) => {
                resp
            },
            Err(_error) => {
                return Err(());
            }
        };
        match file_rw.write(decrypted_bytes.as_ref()) {
            Ok(_resp) => {},
            Err(_error) => {
                return Err(());
            }
        };
    }

    Ok(())
}

/// Used to encrypt/decrypt byte arrays with a given key and optionally nonces
pub struct Cryptor {
    cipher: ChaChaPoly1305<ChaCha20, U12>,
    nonce: [u8; 12]
}

impl Cryptor {
    /// Returns an instance `Cryptor` with the provided arguments
    ///
    /// # Arguments
    /// * `key` - The 256-bit key used for encryption/decryption
    /// * `input_nonce` - Optional, allows you to specicify a nonce (number-used-once)
    /// 
    /// # Notes
    /// Specifying an input_nonce is useful when decrypting a file. You will need to use the same nonce that was used to encrypt the data
    pub fn new(key: [u8; 32], input_nonce: Option<[u8; 12]>) -> Result<Self, ()> {
        // Create ChaCha20Poly1305 cipher using our 256bit key
        let cipher = match ChaCha20Poly1305::new_from_slice(&key) {
            Ok(resp) => {
                resp
            },
            Err(error) => {
                error!("Failed to create an instance of the ChaCha20Poly1305 Cryptor:\n {error}");
                return Err(());
            },
        };

        // Use provided nonce or generate a random one
        let mut nonce = [0u8; 12];
        match input_nonce {
            Some(resp) => {
                nonce = resp;
            },
            None => {
                OsRng.fill_bytes(&mut nonce);
            },
        };
        
        // Return Cryptor instance
        Ok(Self {
            cipher,
            nonce
        })
    }

    /// Encrypts a given slice of bytes
    /// 
    /// # Arguments
    /// * `buffer` - The buffer of bytes that you want encrypted
    /// 
    /// # Returns
    /// Returns a Vector of bytes (Vec\<u8>)
    pub fn encrypt_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>, ()> {
        let encrypted_bytes = match self.cipher.encrypt(&self.nonce.into(), buffer) {
            Ok(resp) => {
                resp
            },
            Err(error) => {
                error!("Failed to encrypt bytes:\n {error}");
                return Err(());
            }
        };
        Ok(encrypted_bytes)
    }

    /// Decrypts a given slice of bytes
    /// 
    /// # Arguments
    /// * `buffer` - The buffer of bytes that you want decrypted
    /// 
    /// # Returns
    /// Returns a Vector of bytes (Vec\<u8>)
    pub fn decrypt_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>, ()> {
        let decrypted_bytes = match self.cipher.decrypt(&self.nonce.into(), buffer) {
            Ok(resp) => {
                resp
            },
            Err(error) => {
                error!("Failed to decrypt bytes:\n {error}");
                return Err(());
            }
        };
        Ok(decrypted_bytes)
    }
}



// AES256 Encryption - Consider adding the ability to utilize AES256 instead of ChaCha20Poly1305 in the future
// let key = output_key_material.clone();
// let cipher = Aes256GcmSiv::new_from_slice(&key).unwrap();
// let nonce = Nonce::from_slice(b"unique nonce");
// let encrypted_message = cipher.encrypt(&nonce, plaintext_message.as_ref()).unwrap();
// let decrypted_message = cipher.decrypt(&nonce, encrypted_message.as_ref()).unwrap();
