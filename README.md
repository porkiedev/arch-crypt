## Short for Archive and Encrypt, Arch-Crypt is a command-line utility for archiving entire directories and encrypting files with passwords.

If you're like me and have little to no trust in cloud storage providers, but you also want a safe way to back up your more critical data, this may suit your needs.

Arch-crypt is a command-line tool. It has only been tested on Windows but there's no reason why it shouldn't work on other operating systems.

Arch-Crypt is capable of archiving directories, encrypting files, and vice-versa.
1. The archiving system makes use of tar. Archived directories are converted to tarballs (.tar)
2. The encryption system makes use of the ChaCha20Poly1305 encryption algorithm along with the password-derivation Argon2id hashing algorithm for the encryption keys.

Arch-Crypt was designed to archive and encrypt small (<100MB) files/directories. That said, I have tested both the archive and encryption functionality with 4.4GB~ files and had no problems. [A theoretical limit of 256 ~ 274.8GB is mentioned below](#probably-useless-notes-for-the-curious-people).



## Usage
`arch-crypt <subcommand>` The current subcommands are `pack`, `unpack`, `encrypt`, and `decrypt`.

You can run `arch-crypt` as well as `arch-crypt <subcommand>` to get a help message for each corresponding module.

>To encrypt a file, use `arch-crypt encrypt <input-file> <output-file>`
>  
> Example: `arch-crypt encrypt picture.png picture.png.enc`

>To archive a directory, use `arch-crypt pack <input-directory> <output-file>`
>  
> Example: `arch-crypt pack my-directory my-directory-archive.tar`

<h4>Note: In both cases, the file extension doesn't matter. However, it does help to add something to the end so you know if a file is encrypted or a tarball.</h4>

## Probably useless notes for the curious people
- During encryption, the input file is read in 32,768-byte-sized blocks. To ensure data integrity and validity (_I.E. ensuring the data wasn't modified or damaged_), the Poly1305 hashing function will calculate a 16-byte 'checksum' and append it to the end of every block of data. This means that the encrypted output file will always be slightly larger than the input file.
- During encryption, the salt that was used by the password-hashing algorithm (_Argon2id_), as well as the nonce used by the encryption algorithm (_ChaCha20_), will be added to the start of the encrypted file. This is needed during decryption so we can reproduce the same hash (_encryption key_) with the same password, as well as decrypt the data later.
- (_I calculated this for fun and my knowledge in cryptology is still very limited, so I could be wrong here!_) Due to the ChaCha20 algorithm using a 32-bit counter in the nonce value (_The nonce is a 96-bit value with a 32-bit counter value_), we can only encrypt a maximum of 4,294,967,295 blocks of data (_The ChaCha20 implementation itself handles data in block-sizes of 512 bits or 64 bytes_). That means that we can't encrypt more than 274.8 Gigabytes of data ([though some other sources mention a limit of 256 GB](https://doc.libsodium.org/advanced/stream_ciphers/chacha20)).

## Feature considerations (no promises)
1. Compression (The compression algorithm is undecided. I am open to ideas!)
