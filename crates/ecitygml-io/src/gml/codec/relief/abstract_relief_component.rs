use crate::Error;
use crate::gml::codec::core::basic_types::GmlIntegerBetween0And3;
use crate::gml::codec::core::{
    deserialize_abstract_space_boundary, serialize_abstract_space_boundary,
};
use crate::gml::util::CombinedCityGmlElement;
use ecitygml_core::model::core::AsAbstractSpaceBoundary;
use ecitygml_core::model::relief::{AbstractReliefComponent, AsAbstractReliefComponent};
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_relief_component(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractReliefComponent, Error> {
    let (abstract_space_boundary_result, parsed_result) = rayon::join(
        || deserialize_abstract_space_boundary(xml_document, spans),
        || de::from_reader::<_, GmlAbstractReliefComponent>(xml_document).map_err(Error::from),
    );
    let abstract_space_boundary = abstract_space_boundary_result?;
    let parsed = parsed_result?;

    let abstract_relief_component = AbstractReliefComponent::from_abstract_space_boundary(
        abstract_space_boundary,
        parsed.lod.into(),
    );
    Ok(abstract_relief_component)
}

pub fn serialize_abstract_relief_component(
    abstract_relief_component: &AbstractReliefComponent,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut parts = serialize_abstract_space_boundary(
        abstract_relief_component.abstract_space_boundary(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlAbstractReliefComponent::from(abstract_relief_component),
        formatting,
    )? {
        parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(parts)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlAbstractReliefComponent {
    #[serde(rename(serialize = "dem:lod", deserialize = "lod"))]
    pub lod: GmlIntegerBetween0And3,
}

impl From<&AbstractReliefComponent> for GmlAbstractReliefComponent {
    fn from(item: &AbstractReliefComponent) -> Self {
        Self {
            lod: item.lod().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::common::LevelOfDetail;
    use ecitygml_core::model::core::AsAbstractFeature;
    use ecitygml_core::model::relief::AsAbstractReliefComponent;
    use egml::io::util::extract_xml_element_spans;
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_tin_relief() {
        let xml_document = "
                <dem:TINRelief gml:id=\"abc\">
                  <dem:lod>2</dem:lod>
                </dem:TINRelief>";

        let spans = extract_xml_element_spans(xml_document.as_ref()).expect("should work");
        let abstract_relief_component =
            deserialize_abstract_relief_component(xml_document.as_ref(), &spans)
                .expect("should work");

        assert_eq!(
            abstract_relief_component.feature_id(),
            &Id::try_from("abc").unwrap()
        );
        assert_eq!(abstract_relief_component.lod(), LevelOfDetail::Two);
    }
}
