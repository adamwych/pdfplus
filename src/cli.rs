use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;
use crate::html;
use crate::generator;
use crate::generator::preprocessor;
use crate::generator::context;
use crate::layout;

fn generate_pdf_from_document(document: Box<html::DocumentRef>, output_file_path: &str) {

    // Pre-process the entire document. Load external fonts, images etc.
    let preproc = preprocessor::Preprocessor::new(Rc::clone(&document));
    let resources_manager = preproc.process_document();

    // Lay out all elements.
    let engine = layout::Engine::new(Rc::clone(&document), Rc::clone(&resources_manager));
    let root_element = engine.process_document();

    let context = context::ConversionContext {
        document: Rc::clone(&document),
        root_element: Some(root_element),
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

    let doc = html::parse_text("
        <div>
            <div style=\"display: block; color: blue;\">
                1
            </div>
            
            <div style=\"display: block; color: red;\">
                2
            </div>
        </div>
            
        <div style=\"display: block; color: green;\">
            3
        </div>
    ");
    
    generate_pdf_from_document(doc, output_file_path);
}

fn print_usage() {
    println!("Usage: ./mpdf <input file path> <output file path>");
}