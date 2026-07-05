use crate::Error;
use crate::gml::util::{XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::AbstractFeature;
use egml::model::base::Id;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);
fn generate_id() -> Id {
    let n = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    Id::try_from(format!("ecitygml-gen-{n}")).expect("ecitygml-gen-{n} is always a valid xsd:ID")
}

pub fn deserialize_abstract_feature(
    xml_document: &[u8],
    _spans: &XmlElementSpans,
) -> Result<AbstractFeature, Error> {
    let mut gml_abstract_feature = egml::io::deserialize_abstract_feature(xml_document)?;

    if gml_abstract_feature.abstract_gml.id.is_none() {
        gml_abstract_feature.abstract_gml.id = Some(generate_id())
        // gml_abstract_feature.abstract_gml.id = Some(Id::from_hashed_bytes(xml_document))
    }

    let abstract_feature = AbstractFeature::from_gml_abstract_feature(gml_abstract_feature);

    Ok(abstract_feature)
}

pub fn serialize_abstract_feature(
    abstract_feature: &AbstractFeature,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();
    xml_node_parts
        .attributes
        .push(("gml:id".to_string(), abstract_feature.id().to_string()));

    if let Some(raw) = serialize_inner(
        egml::io::GmlAbstractFeature::from(abstract_feature.gml_abstract_feature()),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractFeature {}

impl From<&AbstractFeature> for GmlAbstractFeature {
    fn from(_item: &AbstractFeature) -> Self {
        Self {}
    }
}
