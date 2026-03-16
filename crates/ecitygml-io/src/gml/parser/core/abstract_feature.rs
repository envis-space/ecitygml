use crate::Error;
use ecitygml_core::model::core::AbstractFeature;
use egml::model::base::Id;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_feature(xml_document: &[u8]) -> Result<AbstractFeature, Error> {
    let gml: GmlAbstractFeature = de::from_reader(xml_document)?;

    let id = match gml.id.as_ref() {
        Some(s) => Id::try_from(s)?,
        None => Id::from_hashed_bytes(xml_document),
    };

    let mut abstract_feature = AbstractFeature::new(id);
    abstract_feature.set_name(gml.name);

    Ok(abstract_feature)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GmlAbstractFeature {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "gml:name", deserialize = "name"),
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub name: Vec<String>,
}

impl From<&AbstractFeature> for GmlAbstractFeature {
    fn from(item: &AbstractFeature) -> Self {
        Self {
            id: Some(item.id().to_string()),
            name: item.name().to_vec(),
        }
    }
}
