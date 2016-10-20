// Copyright (c) 2016  Sebastian Wiesner <swiesner@lunaryorn.com>

// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License.  You may obtain a copy
// of the License at

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.  See the
// License for the specific language governing permissions and limitations under
// the License.

extern crate env_logger;
extern crate bindgen;
extern crate hyper;

use hyper::client::IntoUrl;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

static EMACS_VERSION: &'static str = "25.1";

fn download_emacs_module_header(dest_file: &Path) -> Result<&Path, Box<Error>> {
    let client = hyper::Client::new();
    let url = format!("https://raw.githubusercontent.\
                       com/emacs-mirror/emacs/emacs-{}/src/emacs-module.h",
                      EMACS_VERSION);
    let mut response = try!(client.get(url.into_url().unwrap()).send());
    let mut sink = try!(File::create(dest_file));
    try!(io::copy(&mut response, &mut sink));
    Ok(dest_file)
}

fn generate_emacs_bindings<'a>(header: &Path, module: &'a Path) -> io::Result<&'a Path> {
    let mut bindings = bindgen::Builder::new(header.to_str().expect("Failed to convert path"));
    // Generate the bindings.  Make sure that we fail on unknown types, include C builtins for
    // varargs support, remove the "emacs_" prefix from the types and convert C enums to Rust
    // constants for easier use as return values.
    let generated_bindings = bindings.forbid_unknown_types()
        .remove_prefix("emacs_")
        .rust_enums(false)
        // Only include relevant headers: The emacs header of course, and stddef.h for `ptrdiff_t`
        .match_pat(header.to_str().unwrap())
        .match_pat("stddef.h")
        .generate()
        .expect("Failed to generate bindings");

    let mut file = try!(File::create(module));
    try!(file.write(b"pub mod emacs {\n"));
    try!(file.write(generated_bindings.to_string().as_bytes()));
    try!(file.write(b"\n}"));
    Ok(module)
}

fn main() {
    // Enable the default environment system to make bindgen print errors when binding generation
    // fails
    env_logger::init().unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let header = Path::new(&out_dir).join("emacs-module.h");
    let module = Path::new(&out_dir).join("emacs.rs");
    generate_emacs_bindings(download_emacs_module_header(&header).unwrap(), &module).unwrap();
    println!("Wrote emacs bindings to {}", module.to_string_lossy());
}
