// Date: Thu Sep 08 2026

// Project: Learning Chapter 13
// Goal: Using File IO: Using BufReader with zero copy
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
use std::io::{self, BufRead, BufReader, Cursor};
use std::str;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");
    let data: &[u8; 31] = b"Rust performance is Awosome ...";
    let mut buffer_reader: BufReader<Cursor<&[u8; 31]>> = io::BufReader::new(Cursor::new(data));

    loop {
        let avalible_buffer: &[u8] = buffer_reader.fill_buf()?;

        if avalible_buffer.is_empty() {
            break;
        }

        let length: usize = avalible_buffer.len();

        println!(
            "value of buffer is:\n{}",
            str::from_utf8(avalible_buffer).unwrap()
        );

        buffer_reader.consume(length);
    }

    println!("\nThe End ...\n");
    Ok(())
}
