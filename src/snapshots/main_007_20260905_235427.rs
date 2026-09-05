// Date: Thu Sep 05 2026

// Project: Learning Chapter 13
// Goal: Using File IO: Read a directory contents
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

use std::fs::{self, DirEntry};
use std::io;

fn main() {
    println!("\n");

    let dir_mem = fs::read_dir("/home/hinata/test");
    let read_dir = match dir_mem {
        Ok(rd) => rd,
        Err(err) => panic!(" -->> An error occurred: {}", err),
    };

    let file_coll: Vec<Result<DirEntry, io::Error>> = read_dir.collect();
    let one: &Result<DirEntry, io::Error> = &file_coll[0];
    let dir_name = match one {
        Ok(de) => de,
        Err(err) => panic!("Error: {}", err),
    };

    // let file_type = dir_name.file_type();
    let path = dir_name.path().into_string();
    let path_name = match path {
        Ok(pn) => pn,
        Err(err) => panic!("Error: {:?}", err),
    };

    println!("first element is: {}", path_name);

    println!("\nThe End ...\n");
}
