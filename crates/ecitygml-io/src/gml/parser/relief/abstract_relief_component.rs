use crate::Error;
use crate::gml::parser::core::parse_abstract_space_boundary;
use ecitygml_core::model::relief::AbstractReliefComponent;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn parse_abstract_relief_component(
    xml_document: &[u8],
) -> Result<AbstractReliefComponent, Error> {
    let abstract_space_boundary = parse_abstract_space_boundary(xml_document)?;
    let gml_abstract_relief_component: GmlAbstractReliefComponent = de::from_reader(xml_document)?;

    let abstract_relief_component = AbstractReliefComponent::new(
        abstract_space_boundary,
        gml_abstract_relief_component.lod.try_into()?,
    );
    Ok(abstract_relief_component)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlAbstractReliefComponent {
    pub lod: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::common::LevelOfDetail;
    use ecitygml_core::model::core::AsAbstractFeature;
    use ecitygml_core::model::relief::AsAbstractReliefComponent;
    use egml::model::base::Id;

    #[test]
    fn test_parse_tin_relief() {
        let xml_document = "
                <dem:TINRelief gml:id=\"abc\">
                  <dem:lod>2</dem:lod>
                </dem:TINRelief>";

        let abstract_relief_component =
            parse_abstract_relief_component(xml_document.as_ref()).expect("should work");

        assert_eq!(
            abstract_relief_component.id(),
            &Id::try_from("abc").unwrap()
        );
        assert_eq!(abstract_relief_component.lod(), LevelOfDetail::Two);
    }
}
