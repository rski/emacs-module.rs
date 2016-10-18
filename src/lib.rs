// Copyright (C) 2016  Sebastian Wiesner <swiesner@lunaryorn.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License along with this program.  If
// not, see <http://www.gnu.org/licenses/>.

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static plugin_is_GPL_compatible: i32 = 0;

include!(concat!(env!("OUT_DIR"), "/emacs.rs"));

use emacs::emacs_runtime;
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn emacs_module_init(runtime: &mut emacs_runtime) -> c_int {
    println!("HELLO THERE FROM RUST");
    0
}
