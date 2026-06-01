use crate::Error;
use crate::gml::util::XmlElementSpans;
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

    let abstract_feature = AbstractFeature::with_gml_abstract_feature(gml_abstract_feature);

    Ok(abstract_feature)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GmlAbstractFeature {
    #[serde(flatten, skip_deserializing)]
    pub abstract_feature: egml::io::GmlAbstractFeature,
}

impl From<&AbstractFeature> for GmlAbstractFeature {
    fn from(item: &AbstractFeature) -> Self {
        Self {
            abstract_feature: item.gml_abstract_feature().into(),
        }
    }
}
