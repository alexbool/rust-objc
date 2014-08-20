#![crate_name = "objc"]
#![crate_type = "lib"]

#![feature(globs, macro_rules, unsafe_destructor)]

extern crate libc;

pub use id::{Id, Identifier, IdVector, IntoIdVector, ShareId};
pub use class_name::{class, ClassName};
pub use declare::{ClassDecl, MethodDecl};
pub use encode::{encode, Encode, Encoding};
pub use message::{to_ptr, ToMessage};
pub use weak::WeakId;

mod macros;

pub mod runtime;
mod id;
mod class_name;
mod declare;
mod encode;
mod message;
mod weak;
pub mod foundation;

// Shim to re-export under the objc:: path for macros
mod objc {
	pub use runtime;
	pub use foundation;
	pub use super::*;
}
