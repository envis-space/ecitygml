use crate::Error;
use crate::gml::parser::core::abstract_feature::{
    GmlAbstractFeature, deserialize_abstract_feature,
};
use chrono::{DateTime, FixedOffset};
use ecitygml_core::model::core::{
    AbstractFeatureWithLifespan, AsAbstractFeature, AsAbstractFeatureWithLifespan,
    AsAbstractFeatureWithLifespanMut,
};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_feature_with_lifespan(
    xml_document: &[u8],
) -> Result<AbstractFeatureWithLifespan, Error> {
    let abstract_feature = deserialize_abstract_feature(xml_document)?;
    let mut abstract_feature_with_lifespan = AbstractFeatureWithLifespan::new(abstract_feature);
    let gml: GmlAbstractFeatureWithLifespan = de::from_reader(xml_document)?;

    abstract_feature_with_lifespan.set_creation_date(gml.creation_date);
    abstract_feature_with_lifespan.set_termination_date(gml.termination_date);
    abstract_feature_with_lifespan.set_valid_from(gml.valid_from);
    abstract_feature_with_lifespan.set_valid_to(gml.valid_to);

    Ok(abstract_feature_with_lifespan)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GmlAbstractFeatureWithLifespan {
    #[serde(flatten, skip_deserializing)]
    pub abstract_feature: GmlAbstractFeature,

    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<DateTime<FixedOffset>>,

    #[serde(rename = "terminationDate", skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<DateTime<FixedOffset>>,

    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<FixedOffset>>,

    #[serde(rename = "validTo", skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<DateTime<FixedOffset>>,
}

impl From<&AbstractFeatureWithLifespan> for GmlAbstractFeatureWithLifespan {
    fn from(item: &AbstractFeatureWithLifespan) -> Self {
        Self {
            abstract_feature: item.abstract_feature().into(),
            creation_date: item.creation_date().copied(),
            termination_date: item.termination_date().copied(),
            valid_from: item.valid_from().copied(),
            valid_to: item.valid_to().copied(),
        }
    }
}
