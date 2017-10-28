// Copyright (C) 2016  Sebastian Wiesner <swiesner@lunaryorn.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License along with this program.  If
// not, see <http://www.gnu.org/licenses/>.

extern crate env_logger;
extern crate bindgen;
extern crate curl;

use curl::easy::Easy;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

static EMACS_VERSION: &'static str = "25.1";

fn download_emacs_module_header(dest_file: &Path) -> Result<(), Box<Error>> {
    let url = format!(
        "https://raw.githubusercontent.\
                       com/emacs-mirror/emacs/emacs-{}/src/emacs-module.h",
        EMACS_VERSION
    );
    let mut client = Easy::new();
    let mut sink = try!(File::create(dest_file));
    client
        .url(&url)
        .and_then(|_| {
            client.write_function(move |data| Ok(sink.write(data).unwrap()))
        })
        .and_then(|_| client.perform())
        .map_err(From::from)
}

fn prepare_emacs_module_header<'a>(orig: &Path, dest: &'a Path) -> io::Result<()> {
    let mut source = try!(File::open(orig));
    let mut contents = String::new();
    try!(source.read_to_string(&mut contents));

    // Skip over everything that could possibly contain 128bit integers, see
    // https://github.com/lunaryorn/emacs-rust-module/issues/5
    let prepared_header = contents
        .lines()
        .filter(|l| !l.contains("intmax_t"))
        .collect::<Vec<_>>()
        .join("\n");

    let mut sink = try!(File::create(&dest));
    sink.write_all(prepared_header.as_bytes())
}

fn generate_emacs_bindings<'a>(header: &Path, module: &'a Path) -> io::Result<()> {
    let mut bindings = bindgen::Builder::new(header.to_str().expect("Failed to convert path"));
    // Make sure that we fail on unknown types
    let generated_bindings = bindings.forbid_unknown_types()
        // Remove the "emacs_" prefix from the types
        .remove_prefix("emacs_")
        // Convert C enums to Rust constants for easier use as return values.
        .rust_enums(false)
        // Only include relevant headers: The emacs header of course, and stddef.h for `ptrdiff_t`
        .match_pat(header.to_str().unwrap())
        .match_pat("stddef.h")
        .generate()
        .expect("Failed to generate bindings");

    let mut file = try!(File::create(module));
    try!(file.write(b"mod generated {\n"));
    try!(file.write(generated_bindings.to_string().as_bytes()));
    try!(file.write(b"\n}"));
    Ok(())
}

fn main() {
    // Enable the default environment system to make bindgen print errors when binding generation
    // fails
    env_logger::init().unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let header = Path::new(&out_dir).join("emacs-module.h");
    let prepared_header = Path::new(&out_dir).join("emacs-module-prepared.h");
    let module = Path::new(&out_dir).join("generated.rs");

    download_emacs_module_header(&header).unwrap();
    prepare_emacs_module_header(&header, &prepared_header).unwrap();
    generate_emacs_bindings(&prepared_header, &module).unwrap();

    // Tell rustc that the library is missing integer support
    println!("cargo:rustc-cfg=nointeger");
    println!("Wrote emacs bindings to {}", module.to_string_lossy());
}
