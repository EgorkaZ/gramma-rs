#![feature(generic_associated_types)]
#![feature(hash_set_entry)]
#![feature(hash_raw_entry)]
#![feature(iter_advance_by)]
#![feature(result_option_inspect)]
#![feature(map_try_insert)]
#![feature(iter_intersperse)]
mod grammar_parser;

pub use grammar_parser::*;

pub mod util;

#[macro_use]
extern crate kiam;

pub mod parser;

pub mod codegen;

pub mod degenerated;
