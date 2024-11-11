use clap::Parser;
mod arguments;
pub use crate::arguments::Args;

fn main() {
    let args = Args::parse();
    println!("{:?}", args)
}