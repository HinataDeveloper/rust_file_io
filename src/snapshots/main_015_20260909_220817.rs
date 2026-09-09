// Date: Thu Sep 09 2026

// Project: Learning Chapter 13
// Goal: Using File IO: Using read_dir method
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

use std::error::Error;
use std::fs::ReadDir;
use std::fs::{self, DirEntry};
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    const PATH_NAME: &str = "/home/hinata/test/";
    let mut dir_path_list: Vec<String> = Vec::new();

    let dir_content_result: Result<ReadDir, io::Error> = fs::read_dir(PATH_NAME);
    let dir_content = match dir_content_result {
        Ok(dr) => dr,
        Err(err) => panic!("An error occurred: {}", err),
    };

    dir_content.for_each(|de_rst| {
        let de: DirEntry = match de_rst {
            Ok(de) => de,
            Err(err) => panic!("An error occurred: {}", err),
        };

        let dir_name: String = match de.path().into_string() {
            Ok(dn) => dn,
            Err(err) => panic!("An error occurred: {:?}", err),
        };

        if de.path().is_dir() {
            dir_path_list.push(dir_name);
        } else {
            println!("File: {}", dir_name);
        }
    });

    dir_path_list.iter().for_each(|elem| {
        println!(" -> {}", elem);
    });

    println!("\nThe End ...\n");
    Ok(())
}
