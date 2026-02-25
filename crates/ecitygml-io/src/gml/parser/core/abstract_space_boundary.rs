use crate::Error;
use crate::gml::parser::core::parse_abstract_city_object;
use ecitygml_core::model::core::AbstractSpaceBoundary;

pub fn parse_abstract_space_boundary(xml_document: &[u8]) -> Result<AbstractSpaceBoundary, Error> {
    let abstract_city_object = parse_abstract_city_object(xml_document)?;
    let abstract_space_boundary = AbstractSpaceBoundary::new(abstract_city_object);

    Ok(abstract_space_boundary)
}
