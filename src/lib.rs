#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(
    unused_must_use,
    unused_macros,
    clippy::borrow_interior_mutable_const,
    clippy::module_inception,
    clippy::collapsible_else_if
)]

pub mod table_const;
pub mod utility;
mod common;
mod metaknight;
pub mod imports;

#[skyline::main(name = "brawl_mk")]
pub fn main() {
    common::install();
    metaknight::install();
}