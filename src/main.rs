#[macro_use]
extern crate serde_derive;

mod cli;
mod data;
mod model;
mod nlp;

fn main() {
    cli::run()
}
