use crate::Error;
use crate::gml::codec::relief::{deserialize_tin_relief, serialize_tin_relief};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::relief::AbstractReliefComponentKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_relief_component_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractReliefComponentKind>, Error> {
    if let Some(span) = spans.first(CityGmlElement::TINRelief.into()) {
        let tin_surface = deserialize_tin_relief(&xml_document[span.start..span.end])?;
        return Ok(Some(tin_surface.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_relief_component_kind(
    abstract_relief_component_kind: &AbstractReliefComponentKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_relief_component_kind {
        AbstractReliefComponentKind::TinRelief(x) => serialize_tin_relief(x, formatting),
    }
}
