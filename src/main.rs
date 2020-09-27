mod cli;
mod html;
mod html_parser;
mod generator;
mod layout_engine;
mod render_engine;
mod preprocessor;
mod context;
mod resources_manager;
mod font;
mod utils;
mod color;
mod css;

fn main() {
    cli::run_cli();
}