use crate::error::Error;
use crate::gml::validate::extracted_information::{
    CityObjectRelation, ExtractedInformation, GmlIdCount,
};
use crate::gml::validate::report::Report;
use quick_xml::Reader;
use quick_xml::de;
use quick_xml::events::Event;
use quick_xml::events::attributes::Attribute;

use std::ops::Deref;

pub fn validate_from_reader(file_content: Vec<u8>) -> Result<Report, Error> {
    let mut extracted_information = ExtractedInformation::default();

    // TODO: improve
    let mut xml_reader = Reader::from_str(std::str::from_utf8(&file_content)?);
    xml_reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    loop {
        match xml_reader.read_event_into(&mut buf) {
            Err(e) => panic!(
                "Error at position {}: {:?}",
                xml_reader.buffer_position(),
                e
            ),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                let element_name = e.name();
                let element_name: String = xml_reader
                    .decoder()
                    .decode(element_name.as_ref())
                    .unwrap()
                    .to_string();

                let id_attribute: Option<Attribute> = e
                    .attributes()
                    .map(|a| a.unwrap())
                    .find(|a| a.key.local_name().as_ref() == "id".as_bytes());

                extracted_information
                    .gml_id_count_per_element_type
                    .entry(element_name.clone())
                    .or_insert(GmlIdCount::default())
                    .increment(id_attribute.is_some());

                if let Some(id_attribute) = id_attribute {
                    let a = id_attribute.decode_and_unescape_value(xml_reader.decoder())?;
                    *extracted_information
                        .gml_id_count
                        .entry(a.deref().to_string())
                        .or_insert(0) += 1;
                };

                if e.name().as_ref() == b"relatedTo" {
                    let read_text: &str = &xml_reader.read_text(e.name()).unwrap();
                    let city_object_relation: CityObjectRelation = de::from_str(read_text).unwrap();

                    extracted_information
                        .city_object_relations
                        .insert(city_object_relation);
                };
            }
            Ok(Event::Empty(e)) => {
                //let read_text: &str = e.unescape().unwrap();

                let href_attribute: Option<String> = e
                    .attributes()
                    .map(|a| a.unwrap())
                    .find(|a| a.key.local_name().as_ref() == "href".as_bytes())
                    .map(|a| {
                        a.decode_and_unescape_value(xml_reader.decoder())
                            .unwrap()
                            .deref()
                            .to_string()
                    });

                match e.name().as_ref() {
                    b"tran:predecessor" => {
                        if let Some(href) = href_attribute {
                            *extracted_information
                                .predecessor_hrefs
                                .entry(href)
                                .or_insert(0) += 1;
                        }
                    }
                    b"tran:successor" => {
                        if let Some(href) = href_attribute {
                            *extracted_information
                                .successor_hrefs
                                .entry(href)
                                .or_insert(0) += 1;
                        }
                    }
                    _ => {}
                };
            }
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    let report = extracted_information.compile_report();
    Ok(report)
}

fn parse_city_object_relation(source_text: &str) -> Result<CityObjectRelation, Error> {
    let c: CityObjectRelation = de::from_str(source_text)?;
    Ok(c)
}

#[cfg(test)]
mod tests {
    use crate::gml::validate_impl::parse_city_object_relation;

    #[test]
    fn parsing_city_object_relation() {
        let _source_text = "<relationType>belongsTo</relationType>\
                 <relatedTo xlink:href=\"#UUID_c930adc7-9e6c-3eea-a377-b31d9d5b6239\"/>";
        let source_text = "<CityObjectRelation><relationType>belongsTo</relationType><relatedTo xlink:href=\"#UUID_c930adc7-9e6c-3eea-a377-b31d9d5b6239\"/></CityObjectRelation>";

        let city_object_relation = parse_city_object_relation(source_text).unwrap();

        assert_eq!(city_object_relation.related_type.value, "belongsTo");
        assert_eq!(
            city_object_relation.related_to.xlink_href,
            "#UUID_c930adc7-9e6c-3eea-a377-b31d9d5b6239"
        );
    }
}
