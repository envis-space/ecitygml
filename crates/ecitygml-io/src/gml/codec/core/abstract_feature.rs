use crate::Error;
use crate::gml::util::{CombinedCityGmlElement, filter_to_gml_element_spans};
use ecitygml_core::model::core::AbstractFeature;
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeParts};
use egml::model::base::{AsAbstractGml, AsAbstractGmlMut, Id};
use egml::model::feature::AsAbstractFeature;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);
fn generate_feature_id() -> Id {
    let n = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    Id::try_from(format!("ecitygml-gen-feature-{n}"))
        .expect("ecitygml-gen-feature-{n} is always a valid xsd:ID")
}

pub fn deserialize_abstract_feature(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractFeature, Error> {
    let gml_spans = filter_to_gml_element_spans(spans);
    let mut gml_abstract_feature =
        egml::io::codec::feature::deserialize_abstract_feature(xml_document, &gml_spans)?;

    if gml_abstract_feature.abstract_gml().id().is_none() {
        gml_abstract_feature
            .abstract_gml_mut()
            .set_id(generate_feature_id());
        // gml_abstract_feature.abstract_gml.id = Some(Id::from_hashed_bytes(xml_document))
    }

    let abstract_feature = AbstractFeature::from_abstract_feature(gml_abstract_feature);

    Ok(abstract_feature)
}

pub fn serialize_abstract_feature(
    abstract_feature: &AbstractFeature,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let xml_node_parts: XmlNodeParts = egml::io::codec::feature::serialize_abstract_feature(
        abstract_feature.abstract_feature(),
        formatting,
    )?;

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractFeature {}

impl From<&AbstractFeature> for GmlAbstractFeature {
    fn from(_item: &AbstractFeature) -> Self {
        Self {}
    }
}
