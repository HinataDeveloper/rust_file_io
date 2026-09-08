// Date: Thu Sep 08 2026

// Project: Learning Chapter 13
// Goal: Using File IO:
// Dependency: Without dependency

// rustc 1.100.0-nightly (5a2be9f5f 2026-09-06)
// binary: rustc
// commit-hash: 5a2be9f5f075d31e3ca5526b5b029881ce441253
// commit-date: 2026-09-06
// host: x86_64-unknown-linux-gnu
// release: 1.100.0-nightly
// LLVM version: 23.1.1

// cargo 1.100.0-nightly (3c0b53475 2026-09-04)
// release: 1.100.0-nightly
// commit-hash: 3c0b534756e166d12eb9fd2e1abfe5b42ac6101e
// commit-date: 2026-09-04
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.1.13-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    // Specify required buffer for BufReader
    const TEMP_BUF: usize = 64 * 1024;

    const PATH_NAME: &str = "/home/hinata/test/temp.txt";
    let target_file: File = File::open(PATH_NAME)?;

    let mut middle_buffer: String = String::new();

    // Using custom buffer
    let mut buffer_reader: io::BufReader<File> =
        io::BufReader::with_capacity(TEMP_BUF, target_file);

    loop {
        middle_buffer.clear();

        let how_much_byte: usize = buffer_reader.read_line(&mut middle_buffer)?;
        if how_much_byte == 0 {
            break;
        }
        println!(" ->> {}", middle_buffer);
    }

    println!("\nThe End ...\n");
    Ok(())
}
