// Copyright (C) 2016  Sebastian Wiesner <swiesner@lunaryorn.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License along with this program.  If
// not, see <http://www.gnu.org/licenses/>.

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub use generated::{emacs_runtime, emacs_env};
pub use generated::{emacs_funcall_exit, emacs_funcall_exit_emacs_funcall_exit_return,
                    emacs_funcall_exit_emacs_funcall_exit_throw};
pub use generated::emacs_arity_emacs_variadic_function;
