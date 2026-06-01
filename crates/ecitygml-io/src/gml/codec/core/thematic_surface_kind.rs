use crate::Error;
use crate::gml::codec::construction::deserialize_construction_surface_kind;
use crate::gml::codec::transportation::{
    deserialize_auxiliary_traffic_area, deserialize_traffic_area,
};
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::core::ThematicSurfaceKind;

pub fn deserialize_thematic_surface_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<ThematicSurfaceKind>, Error> {
    if let Some(x) = deserialize_construction_surface_kind(xml_document, spans)? {
        return Ok(Some(ThematicSurfaceKind::ConstructionSurfaceKind(x)));
    }

    if let Some(span) = spans.first(XmlElement::TrafficArea) {
        let traffic_area = deserialize_traffic_area(&xml_document[span.start..span.end])?;
        return Ok(Some(ThematicSurfaceKind::TrafficArea(traffic_area)));
    }
    if let Some(span) = spans.first(XmlElement::AuxiliaryTrafficArea) {
        let auxiliary_traffic_area =
            deserialize_auxiliary_traffic_area(&xml_document[span.start..span.end])?;
        return Ok(Some(ThematicSurfaceKind::AuxiliaryTrafficArea(
            auxiliary_traffic_area,
        )));
    }

    Ok(None)
}
