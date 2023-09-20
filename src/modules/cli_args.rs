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


use clap::{arg, Command};


pub fn arguments() -> Command {
    Command::new(file!())
    .about("A utility to archive and crypt entire directories")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
        Command::new("pack") // Pack a directory into a tarball
            .about("Pack a directory into a tarball")
            .arg(arg!(<INPUT_DIRECTORY> "The directory to archive"))
            .arg_required_else_help(true)
            .arg(arg!(<OUTPUT_FILE> "The name of the output file"))
            .arg_required_else_help(true)
    )
    .subcommand(
        Command::new("unpack") // Unpack a tarball
            .about("Unpack a tarball")
            .arg(arg!(<INPUT_FILE> "The tarball to unpack"))
            .arg_required_else_help(true)
            .arg(arg!(<OUTPUT_DIRECTORY> "The directory to unpack the tarball contents in"))
            .arg_required_else_help(true)
    )
    .subcommand(
        Command::new("encrypt") // Encrypt a file
            .about("Encrypt a file")
            .arg(arg!(<INPUT_FILE> "The file to encrypt"))
            .arg_required_else_help(true)
            .arg(arg!(<OUTPUT_FILE> "The name of the output file"))
            .arg_required_else_help(true)
    )
    .subcommand(
        Command::new("decrypt") // Decrypt a file
            .about("Decrypt a file")
            .arg(arg!(<INPUT_FILE> "The file to decrypt"))
            .arg_required_else_help(true)
            .arg(arg!(<OUTPUT_FILE> "The name of the output file"))
            .arg_required_else_help(true)
    )
}
