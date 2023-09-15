## Short for Archive and Encrypt, Arch-Crypt is a command-line utility for archiving entire directories and encrypting files with passwords.

If you're like me and have little to no trust in cloud storage providers, but you also want a safe way to back up your data, this may suit your needs.

This is a command-line tool. It has only been tested on Windows but there's no reason why it shouldn't work on other architectures.

Arch-Crypt has 2 main functions. Archiving directories, Encrypting files, and vice-versa.
- The archiving system makes use of tar. Archived directories are converted to tarballs (.tar)
- The encryption system makes use of the ChaCha20Poly1305 encryption algorithm, and the password-derived Argon2id hashing algorithm for the encryption keys.



## Usage
`arch-crypt <command>` the supported commands are `pack`, `unpack`, `encrypt`, `decrypt`

To encrypt a file, you can run `arch-crypt encrypt <input-file> <output-file>`
- Ex: `arch-crypt encrypt picture.png picture.png.enc`

To archive a directory, you can run `arch-crypt pack <input-directory> <output-file>`
- Ex: `arch-crypt pack my-directory my-directory-archive.tar`

<h4>Note: In both cases, the file extension doesn't matter, though it does help to add something to the end so you know if a file is an archive or encrypted.</h4>
