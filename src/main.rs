mod cli;
mod html;
mod generator;
mod layout_engine;
mod render_engine;
mod preprocessor;
mod context;
mod resources_manager;
mod font;
mod utils;
mod png_render_engine;

fn main() {
    cli::run_cli();
}