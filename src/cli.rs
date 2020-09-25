#![allow(unused)]

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;
use std::cell::RefCell;
use crate::html;
use crate::generator;
use crate::layout_engine;
use crate::preprocessor;
use crate::context;
use crate::html_parser;

fn generate_pdf_from_document(document: Box<html::DocumentRef>, output_file_path: &str) {

    // Pre-process the entire document. Load external fonts, images etc.
    let preproc = preprocessor::Preprocessor::new(Rc::clone(&document));
    let resources_manager = preproc.process_document();

    // Lay out all elements.
    let engine = layout_engine::Engine::new(Rc::clone(&document), Rc::clone(&resources_manager));
    let layout_result = engine.process_document();

    let context = context::ConversionContext {
        document: Rc::clone(&document),
        layout_result: Some(layout_result),
        resources_manager: Some(resources_manager)
    };

    // Render to PDF or PNG.
    let pdf = generator::generate_pdf(context);
    let mut writer = BufWriter::new(File::create(output_file_path).unwrap());
    match pdf.save(&mut writer) {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error. {}", e)
    }
}

pub fn run_cli() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage();
        return;
    }

    let _input_file_path = &args[1];
    let output_file_path = &args[2];

    let mut doc = html_parser::parse_text("<div style=\"background-color: red;\">Hello world from actual HTML!</div>");
    // println!("{:#?}", doc);
    generate_pdf_from_document(doc, output_file_path);
}

fn print_usage() {
    println!("Usage: ./mpdf <input file path> <output file path>");
}