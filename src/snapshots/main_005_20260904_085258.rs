// Date: Thu Sep 04 2026

// Project: Learning Chapter 13
// Goal: Using File IO: Remove directory
// Dependency: Without dependency

// rustc 1.100.0-nightly (a69a63265 2026-09-03)
// binary: rustc
// commit-hash: a69a63265cfd9e006d43137f98301b8d274ad4c9
// commit-date: 2026-09-03
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
// os: Fedora 44.0.0 [64-bit

// Kernel Version: 7.1.12-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use std::fs;

fn main() {
    println!("\n");

    let resultant = fs::remove_dir("/home/hinata/Raphael");
    match resultant {
        Ok(_) => println!("Directory was removed successfully ..."),
        Err(err) => eprintln!("Error: {}", err),
    }

    println!("\nThe End ...\n");
}
