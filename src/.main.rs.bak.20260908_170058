// Date: Thu Sep 08 2026

// Project: Learning Chapter 13
// Goal: Using File IO: Using BufReader with Zero-Copy
// Dependency: Without dependency

// rustc 1.100.0-nightly (cea272fa3 2026-09-07)
// binary: rustc
// commit-hash: cea272fa356e94bd2ee2cadf376630aa0683867a
// commit-date: 2026-09-07
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

use std::{
    error::Error,
    io::{BufRead, BufReader, Cursor},
    str::from_utf8,
    thread,
    time::Duration,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let mut counter = 0_u32;

    let data: &[u8; 75] =
        b"How do you think about Rust? do you believe future will be belongs to Rust ";
    let cursor: Cursor<&[u8; 75]> = Cursor::new(data);
    let mut buffer_reader: BufReader<Cursor<&[u8; 75]>> = BufReader::new(cursor);

    loop {
        counter += 1;

        // avalable_buffer has a dynamic size
        let avalable_buffer: &[u8] = buffer_reader.fill_buf()?;
        if avalable_buffer.is_empty() {
            break;
        }

        let length: usize = avalable_buffer.len();
        println!("value of length is: {}", length);
        let result: &str = from_utf8(avalable_buffer)?;

        println!("{}", result);

        buffer_reader.consume(length);
        thread::sleep(Duration::from_secs(1));
    }

    println!("value of counter is: {}", counter);

    println!("\nThe End ...\n");
    Ok(())
}
