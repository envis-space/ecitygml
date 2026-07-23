use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_feature_with_lifespan, serialize_abstract_feature_with_lifespan,
};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::core::{AbstractAppearance, AsAbstractFeatureWithLifespan};
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeParts};

pub fn deserialize_abstract_appearance(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractAppearance, Error> {
    let abstract_feature_with_lifespan =
        deserialize_abstract_feature_with_lifespan(xml_document, spans)?;

    let abstract_appearance =
        AbstractAppearance::from_abstract_feature_with_lifespan(abstract_feature_with_lifespan);

    Ok(abstract_appearance)
}

pub fn serialize_abstract_appearance(
    abstract_appearance: &AbstractAppearance,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let xml_node_parts = serialize_abstract_feature_with_lifespan(
        abstract_appearance.abstract_feature_with_lifespan(),
        formatting,
    )?;

    Ok(xml_node_parts)
}
