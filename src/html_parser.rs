use crate::html;
use crate::css::{parse_inline as parse_inline_css};
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document};
use markup5ever_rcdom as rcdom;

fn walk(handle: &rcdom::Handle, target: &mut html::DocumentRef) -> Option<usize> {
    let node = handle;
    let mut result: Option<usize> = None;

    match node.data {
        rcdom::NodeData::Document => {
            result = Some(target.borrow_mut().get_root_index());
        }

        rcdom::NodeData::Text { ref contents } => {
            let text_elem_idx = target.borrow_mut().create_text_element(&contents.borrow());
            result = Some(text_elem_idx);
        }

        rcdom::NodeData::Element { ref name, ref attrs, .. } => {
            let mut doc = target.borrow_mut();
            let element_idx = doc.create_element(&name.local.to_string());
            let element = doc.get_element(element_idx);

            for attribute in attrs.borrow().iter() {
                let attr_name = &attribute.name.local.to_string();
                let attr_value = &attribute.value.to_string();

                if attr_name == "style" {
                    let declarations = parse_inline_css(attr_value);
                    for declaration in declarations {
                        element.add_style_property(declaration.name.as_str(), declaration.value);
                    }
                } else {
                    element.add_attribute(attr_name, attr_value);
                }
            }

            result = Some(element_idx);
        }

        _ => { }
    }

    if result.is_some() {
        let result_idx = result.unwrap();
        for child in node.children.borrow().iter() {
            if let Some(child_element_idx) = walk(child, target) {
                target.borrow_mut().add_element(child_element_idx, result_idx);
            }
        }
    }

    return result;
}

pub fn parse_text(text: &str) -> Box<html::DocumentRef> {
    let mut document = Box::new(html::Document::new());

    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let dom = parse_document(rcdom::RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut text.as_bytes())
        .unwrap();

    walk(&dom.document, &mut document);

    return document;
}