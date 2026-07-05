use crate::Error;
use crate::gml::codec::core::serialize_city_model;
use crate::gml::write::Formatting;
use ecitygml_core::model::core::CityModel;
use quick_xml::events::{BytesDecl, Event};
use std::io::Write;

pub fn serialize<W: Write>(
    writer: &mut W,
    city_model: CityModel,
    formatting: Formatting,
) -> Result<(), Error> {
    let mut xml_writer = quick_xml::Writer::new(writer);

    xml_writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )))?;
    if formatting != Formatting::Compact {
        xml_writer.get_mut().write_all(b"\n")?;
    }

    serialize_city_model(&mut xml_writer, city_model, formatting)?;

    Ok(())
}
