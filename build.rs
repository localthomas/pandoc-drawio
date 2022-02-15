/*
SPDX-FileCopyrightText: 2022 localthomas
SPDX-License-Identifier: MIT OR Apache-2.0
*/

use std::io::prelude::*;
use std::{fs::File, process::Command};

fn main() {
    // only run this build script for generating the license information on relevant files
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=about.toml");
    println!("cargo:rerun-if-changed=about.hbs");

    let out = Command::new("cargo-about")
        .arg("generate")
        .arg("about.hbs")
        .output()
        .expect("failed to execute generate license command! Is cargo-about installed?");

    if !out.status.success() {
        eprintln!(
            "{}",
            String::from_utf8(out.stderr).expect("could not parse output as text")
        );
        panic!("failed to generate license information");
    }

    let mut file = File::create("license.html").expect("could not open file for license.html");
    file.write_all(&out.stdout)
        .expect("could not write to license.html");
}
