#![allow(unused)]

use std::env;
use std::fs::File;
use std::io::BufWriter;
use crate::html;
use crate::generator;
use crate::layout_engine;

pub fn run_cli() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage();
        return;
    }

    let _input_file_path = &args[1];
    let output_file_path = &args[2];

    // Create an empty document.
    let mut doc = html::Document::new();

    // Add two test <div>s.
    let test_div_idx = doc.create_element("div");
    doc.get_element(test_div_idx).add_style_property("width", "64");
    doc.get_element(test_div_idx).add_style_property("height", "48");
    doc.add_element_to_root(test_div_idx);

    let test_div2_idx = doc.create_element("div");
    doc.get_element(test_div2_idx).add_style_property("width", "32");
    doc.get_element(test_div2_idx).add_style_property("height", "32");
    doc.add_element_to_root(test_div2_idx);

    // Lay out all elements.
    let mut engine = layout_engine::Engine::new(doc);
    let layout_result = engine.process_document();

    // Render to PDF.
    let pdf = generator::generate_pdf(layout_result);
    let mut writer = BufWriter::new(File::create(output_file_path).unwrap());
    match pdf.save(&mut writer) {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error. {}", e)
    }
}

fn print_usage() {
    println!("Usage: ./mpdf <input file path> <output file path>");
}