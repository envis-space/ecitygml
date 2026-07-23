use crate::Error;
use crate::gml::codec::building::{
    deserialize_abstract_building_kind, serialize_abstract_building_kind,
};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::construction::AbstractConstructionKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_construction_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractConstructionKind>, Error> {
    let abstract_building_kind = deserialize_abstract_building_kind(xml_document, spans)?;
    if let Some(x) = abstract_building_kind {
        return Ok(Some(AbstractConstructionKind::AbstractBuildingKind(x)));
    }

    Ok(None)
}

pub fn serialize_abstract_construction_kind(
    abstract_construction_kind: &AbstractConstructionKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_construction_kind {
        AbstractConstructionKind::AbstractBuildingKind(x) => {
            serialize_abstract_building_kind(x, formatting)
        }
    }
}
