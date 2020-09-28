mod css;
mod cli;
mod generator;
mod html;
mod layout;
mod utils;

#[macro_use]
extern crate lazy_static;

fn main() {
    cli::run_cli();
}