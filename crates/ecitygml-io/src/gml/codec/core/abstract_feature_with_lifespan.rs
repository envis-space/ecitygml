use crate::Error;
use crate::gml::codec::core::abstract_feature::{
    deserialize_abstract_feature, serialize_abstract_feature,
};
use crate::gml::util::{XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use crate::gml::write::Formatting;
use chrono::{DateTime, FixedOffset};
use ecitygml_core::model::core::{
    AbstractFeatureWithLifespan, AsAbstractFeature, AsAbstractFeatureWithLifespan,
    AsAbstractFeatureWithLifespanMut,
};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_feature_with_lifespan(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractFeatureWithLifespan, Error> {
    let (abstract_feature_result, gml_result) = rayon::join(
        || deserialize_abstract_feature(xml_document, spans),
        || de::from_reader::<_, GmlAbstractFeatureWithLifespan>(xml_document).map_err(Error::from),
    );
    let abstract_feature = abstract_feature_result?;
    let gml = gml_result?;

    let mut abstract_feature_with_lifespan =
        AbstractFeatureWithLifespan::from_abstract_feature(abstract_feature);
    abstract_feature_with_lifespan.set_creation_date(gml.creation_date);
    abstract_feature_with_lifespan.set_termination_date(gml.termination_date);
    abstract_feature_with_lifespan.set_valid_from(gml.valid_from);
    abstract_feature_with_lifespan.set_valid_to(gml.valid_to);

    Ok(abstract_feature_with_lifespan)
}

pub fn serialize_abstract_feature_with_lifespan(
    abstract_feature_with_lifespan: &AbstractFeatureWithLifespan,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = serialize_abstract_feature(
        abstract_feature_with_lifespan.abstract_feature(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlAbstractFeatureWithLifespan::from(abstract_feature_with_lifespan),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractFeatureWithLifespan {
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
            creation_date: item.creation_date().copied(),
            termination_date: item.termination_date().copied(),
            valid_from: item.valid_from().copied(),
            valid_to: item.valid_to().copied(),
        }
    }
}
