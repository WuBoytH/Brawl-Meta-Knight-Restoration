#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(
    unused_must_use,
    unused_macros
)]

pub mod table_const;
pub mod utility;
mod common;
mod metaknight;
mod imports;

#[skyline::main(name = "brawl_mk")]
pub fn main() {
    common::install();
    metaknight::install();
}