use crate::Error;
use crate::gml::codec::core::deserialize_abstract_space;
use crate::gml::util::XmlElementSpans;
use ecitygml_core::model::core::AbstractLogicalSpace;

pub fn deserialize_abstract_logical_space(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractLogicalSpace, Error> {
    let abstract_space = deserialize_abstract_space(xml_document, spans)?;
    let abstract_logical_space = AbstractLogicalSpace::new(abstract_space);

    Ok(abstract_logical_space)
}
