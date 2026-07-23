use crate::Error;
use crate::gml::codec::building::{deserialize_abstract_building, serialize_abstract_building};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::building::{AsAbstractBuilding, BuildingPart};
use egml::io::util::{Formatting, XmlNode, extract_xml_element_spans};

pub fn deserialize_building_part(xml_document: &[u8]) -> Result<BuildingPart, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_building = deserialize_abstract_building(xml_document, &spans)?;
    let building_part = BuildingPart::from_abstract_building(abstract_building);

    Ok(building_part)
}

pub fn serialize_building_part(
    building: &BuildingPart,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let xml_node_parts = serialize_abstract_building(building.abstract_building(), formatting)?;

    Ok(XmlNode::new(
        CityGmlElement::BuildingPart.into(),
        xml_node_parts,
    ))
}
