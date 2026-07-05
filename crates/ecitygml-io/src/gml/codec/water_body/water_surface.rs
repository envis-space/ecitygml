use crate::Error;
use crate::gml::codec::water_body::abstract_water_boundary_surface::{
    deserialize_abstract_water_boundary_surface, serialize_abstract_water_boundary_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, extract_xml_element_spans};
use crate::gml::write::Formatting;
use ecitygml_core::model::water_body::{AsAbstractWaterBoundarySurface, WaterSurface};
use egml::io::GmlCode;
use serde::{Deserialize, Serialize};

pub fn deserialize_water_surface(xml_document: &[u8]) -> Result<WaterSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_water_boundary_surface =
        deserialize_abstract_water_boundary_surface(xml_document, &spans)?;
    let parsed =
        quick_xml::de::from_reader::<_, GmlWaterSurface>(xml_document).map_err(Error::from)?;

    let mut water_surface =
        WaterSurface::from_abstract_water_boundary_surface(abstract_water_boundary_surface);
    water_surface.set_water_level(parsed.water_level.map(Into::into));

    Ok(water_surface)
}

pub fn serialize_water_surface(
    water_surface: &WaterSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let xml_node_parts = serialize_abstract_water_boundary_surface(
        water_surface.abstract_water_boundary_surface(),
        formatting,
    )?;

    Ok(XmlNode::new(XmlElement::WaterSurface, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlWaterSurface {
    #[serde(
        rename(serialize = "wtr:waterLevel", deserialize = "waterLevel"),
        skip_serializing_if = "Option::is_none"
    )]
    pub water_level: Option<GmlCode>,
}

impl From<&WaterSurface> for GmlWaterSurface {
    fn from(item: &WaterSurface) -> Self {
        Self {
            water_level: item.water_level().map(Into::into),
        }
    }
}
