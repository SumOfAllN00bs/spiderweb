#![feature(test)]
extern crate test;
mod commands;
mod tools;
pub use tools::*;
pub use commands::*;


fn main() {
    // println!("{}", caesar(":;[\\^~#`X4KJS58yZ8 ld7nS2ZrZp XjjV8mxLkH :3 :) :-| B-izsToP0dR6B baxo1l0jup", 13));
}

#[cfg(test)]
mod tests;