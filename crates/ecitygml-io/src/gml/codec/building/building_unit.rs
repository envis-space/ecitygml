use crate::Error;
use crate::gml::codec::building::abstract_building_subdivision::{
    deserialize_abstract_building_subdivision, serialize_abstract_building_subdivision,
};
use egml::io::util::extract_xml_element_spans;
use egml::io::util::{Formatting, XmlNode};

use crate::gml::util::CityGmlElement;
use ecitygml_core::model::building::{AsAbstractBuildingSubdivision, BuildingUnit};

pub fn deserialize_building_unit(xml_document: &[u8]) -> Result<BuildingUnit, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_building_subdivision =
        deserialize_abstract_building_subdivision(xml_document, &spans)?;
    let building_unit =
        BuildingUnit::from_abstract_building_subdivision(abstract_building_subdivision);

    Ok(building_unit)
}

pub fn serialize_building_unit(
    building_unit: &BuildingUnit,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let xml_node_parts = serialize_abstract_building_subdivision(
        building_unit.abstract_building_subdivision(),
        formatting,
    )?;
    Ok(XmlNode::new(
        CityGmlElement::BuildingUnit.into(),
        xml_node_parts,
    ))
}
