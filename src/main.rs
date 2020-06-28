mod commands;
mod tools;
pub use tools::*;
pub use commands::*;


fn main() {
    //println!("{}", caesar("lol", 1));
}

#[cfg(test)]
mod tests;