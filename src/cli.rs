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

fn generate_pdf_from_document(document: html::DocumentRef, output_file_path: &str, to_pdf: bool) {

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
    let pdf = generator::generate_pdf(context, to_pdf);

    if to_pdf {
        let mut writer = BufWriter::new(File::create(output_file_path).unwrap());
        match pdf.save(&mut writer) {
            Ok(_) => println!("Done."),
            Err(e) => println!("Error. {}", e)
        }
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

    // Create an empty document.
    let mut doc = html::Document::new();

    let test_text_idx = doc.borrow_mut().create_text_element("Lorem ipsum dolor sit amet!");
    doc.borrow_mut().get_element(test_text_idx).add_style_property("font", "Roboto-Thin");
    doc.borrow_mut().add_element_to_root(test_text_idx);

    let test_text2_idx = doc.borrow_mut().create_text_element("Hello, world!");
    doc.borrow_mut().get_element(test_text2_idx).add_style_property("font", "Arial");
    doc.borrow_mut().add_element_to_root(test_text2_idx);

    let test_text3_idx = doc.borrow_mut().create_text_element("Lorem ipsum dolor sit amet!");
    doc.borrow_mut().get_element(test_text3_idx).add_style_property("font", "Roboto-Black");
    doc.borrow_mut().add_element_to_root(test_text3_idx);

    generate_pdf_from_document(doc.clone(), output_file_path, true);
    generate_pdf_from_document(doc, output_file_path, false);
}

fn print_usage() {
    println!("Usage: ./mpdf <input file path> <output file path>");
}