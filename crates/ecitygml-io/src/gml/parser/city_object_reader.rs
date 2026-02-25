use crate::Error;
use crate::gml::parser::util::{CityObjectSpan, city_object_class_from_bytes, parse_city_object};
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::core::CityObjectKind;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Span;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;

#[derive(Debug)]
struct ElementContext {
    start_index: u64,
    name: Vec<u8>,
}

pub fn read_city_objects(
    xml_document: &[u8],
    relevant: HashSet<CityObjectClass>,
) -> Result<Vec<CityObjectKind>, Error> {
    let mut reader = Reader::from_reader(xml_document);
    reader.config_mut().trim_text(true);

    let mut element_stack: Vec<ElementContext> = Vec::new();
    let mut city_object_spans: Vec<CityObjectSpan> = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let context = ElementContext {
                    start_index: reader.buffer_position(),
                    name: e.name().as_ref().to_vec(),
                };
                element_stack.push(context);

                /*print!("for current stack (added last): ");
                for c in element_stack.iter() {
                    print!(
                        "(start {}, name {})",
                        c.start_index,
                        String::from_utf8_lossy(&c.name)
                    )
                }
                println!();*/

                let city_object_class = city_object_class_from_bytes(e.local_name().as_ref()).ok();
                match city_object_class {
                    None => {}
                    Some(x) => {
                        if relevant.contains(&x) {
                            let parent_element_context = element_stack.iter().rev().nth(1).unwrap();
                            let span = reader.read_to_end(QName(&parent_element_context.name))?;
                            let parent_span = Span {
                                start: parent_element_context.start_index,
                                end: span.end,
                            };

                            let city_object_span = CityObjectSpan::new(
                                city_object_class.expect("should be set"),
                                parent_span,
                            );
                            city_object_spans.push(city_object_span);
                            element_stack.pop();
                        } else {
                            // reader.read_to_end(e.name())?;
                        }
                    }
                }
            }
            Ok(Event::End(_e)) => {
                /*print!("for current stack (will remove last): ");
                for c in element_stack.iter() {
                    print!(
                        "(start {}, name {})",
                        c.start_index,
                        String::from_utf8_lossy(&c.name)
                    )
                }
                println!();*/
                element_stack.pop();
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Text(_e)) => {}
            _ => (),
        }
    }

    /* print!("at the end: ");
    for c in element_stack.iter() {
        print!(
            "(start {}, name {})",
            c.start_index,
            String::from_utf8_lossy(&c.name)
        )
    }
    println!();*/

    let parsed_city_objects = city_object_spans
        .into_par_iter()
        .map(|location| parse_city_object(xml_document, location))
        .collect::<Result<Vec<CityObjectKind>, Error>>()?;

    Ok(parsed_city_objects)
}
