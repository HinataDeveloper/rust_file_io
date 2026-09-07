// Date: Thu Sep 07 2026

// Project: Learning Chapter 13
// Goal: Using File IO: Using BufReader
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
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    const PATH_NAME: &str = "/home/hinata/test/temp.txt";
    let my_file: File = File::open(PATH_NAME)?;

    let mut buffer_reader: BufReader<File> = io::BufReader::new(my_file);
    let mut chunk: String = String::new();

    loop {
        chunk.clear();
        let byte_read: usize = buffer_reader.read_line(&mut chunk)?;

        if byte_read == 0 {
            break;
        }

        print!(" -> {}", chunk);
    }

    println!("\nThe End ...\n");
    Ok(())
}
