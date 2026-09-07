// Date: Thu Sep 07 2026

// Project: Learning Chapter 13
// Goal: Using File IO: Using BufReader
// Dependency: Without dependency

// rustc 1.100.0-nightly (0ed41eb41 2026-09-04)
// binary: rustc
// commit-hash: 0ed41eb4142dda2df61eb1145a312c1a9d62eb56
// commit-date: 2026-09-04
// host: x86_64-unknown-linux-gnu
// release: 1.100.0-nightly
// LLVM version: 23.1.1

// cargo 1.100.0-nightly (b2e9d5f9d 2026-09-02)
// release: 1.100.0-nightly
// commit-hash: b2e9d5f9db3fb1c454ab84f10c16508984a266e2
// commit-date: 2026-09-02
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.1.13-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    println!("\n");

    const PATH_NAME: &str = "/home/hinata/test/temp.txt";

    let my_file: File = File::open(PATH_NAME)?;
    let buffer_reader: BufReader<File> = io::BufReader::new(my_file);

    for (index, line_result) in buffer_reader.lines().enumerate() {
        let line = line_result?;
        println!("line:{} -> {}", index + 1, line);
    }

    println!("\nThe End ...\n");
    Ok(())
}
