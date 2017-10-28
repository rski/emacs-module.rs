// Copyright (C) 2016  Sebastian Wiesner <swiesner@lunaryorn.com>

// This program is free software: you can redistribute it and/or modify it
// under the terms of the
// GNU General Public License as published by the Free Software Foundation,
// either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE.  See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License along with
// this program.  If
// not, see <http://www.gnu.org/licenses/>.

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static plugin_is_GPL_compatible: i32 = 0;

extern crate emacs_module_sys as sys;

use std::ffi::CString;
use std::os::raw::c_int;

unsafe fn message(env: &mut sys::env, text: &str) {
    let message_symbol = env.intern.unwrap()(env, CString::new("message").unwrap().as_ptr());
    let emacs_format =
        env.make_string.unwrap()(env, "%s".as_ptr() as (*const i8), "%s".len() as isize);
    let emacs_text =
        env.make_string.unwrap()(env, text.as_ptr() as (*const i8), text.len() as isize);
    let mut args = [emacs_format, emacs_text];
    env.funcall.unwrap()(env, message_symbol, args.len() as isize, args.as_mut_ptr());
}

#[no_mangle]
pub extern "C" fn emacs_module_init(runtime: *mut sys::runtime) -> c_int {
    assert!(!runtime.is_null());
    println!("HELLO THERE FROM RUST");

    unsafe {
        let env = (*runtime).get_environment.unwrap()(runtime);
        assert!(!env.is_null());
        message(&mut *env, "HELLO THERE FROM EMACS");
        sys::funcall_exit_return as i32
    }
}
