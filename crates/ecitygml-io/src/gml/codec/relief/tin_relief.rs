use crate::Error;
use crate::gml::codec::relief::abstract_relief_component::{
    deserialize_abstract_relief_component, serialize_abstract_relief_component,
};
use crate::gml::codec::relief::tin_property::GmlTinProperty;
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner};
use crate::gml::write::Formatting;
use ecitygml_core::model::relief::{AsAbstractReliefComponent, TinProperty, TinRelief};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_tin_relief(xml_document: &[u8]) -> Result<TinRelief, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_relief_component_result, parsed_result) = rayon::join(
        || deserialize_abstract_relief_component(xml_document, &spans),
        || de::from_reader::<_, GmlTinRelief>(xml_document).map_err(Error::from),
    );
    let abstract_relief_component = abstract_relief_component_result?;
    let parsed = parsed_result?;

    let mut tin_relief = TinRelief::from_abstract_relief_component(abstract_relief_component);
    let tin: Option<TinProperty> = parsed.tin.map(|x| x.try_into()).transpose()?;
    tin_relief.set_tin(tin);

    Ok(tin_relief)
}

pub fn serialize_tin_relief(
    tin_relief: &TinRelief,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts =
        serialize_abstract_relief_component(tin_relief.abstract_relief_component(), formatting)?;

    if let Some(raw) = serialize_inner(GmlTinRelief::from(tin_relief), formatting)? {
        parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(XmlElement::TINRelief, parts))
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlTinRelief {
    #[serde(
        rename(serialize = "dem:tin", deserialize = "tin"),
        skip_serializing_if = "Option::is_none"
    )]
    pub tin: Option<GmlTinProperty>,
}

impl From<&TinRelief> for GmlTinRelief {
    fn from(item: &TinRelief) -> Self {
        Self {
            tin: item.tin().map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::common::LevelOfDetail;
    use ecitygml_core::model::core::AsAbstractFeature;
    use ecitygml_core::model::relief::AsAbstractReliefComponent;
    use egml::model::base::Id;
    use egml::model::geometry::primitives::AsSurface;

    #[test]
    fn test_deserialize_tin_relief() {
        let xml_document = "
            <dem:TINRelief gml:id=\"abc\">
                <dem:lod>3</dem:lod>
                <dem:tin>
                    <gml:TriangulatedSurface gml:id=\"ground\">
                      <gml:patches>
                        <gml:Triangle>
                          <gml:exterior>
                            <gml:LinearRing gml:id=\"Geo28109128\">
                              <gml:posList>513037.352708 5403284.890495 247.27 513034.424098 5403297.424959 247.19 513034.423981 5403287.429362 247.3 513037.352708 5403284.890495 247.27</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Triangle>
                      </gml:patches>
                    </gml:TriangulatedSurface>
                </dem:tin>
            </dem:TINRelief>";

        let tin_relief = deserialize_tin_relief(xml_document.as_bytes()).expect("should work");

        assert_eq!(tin_relief.id(), &Id::try_from("abc").unwrap());
        assert_eq!(tin_relief.lod(), LevelOfDetail::Three);
        assert_eq!(
            tin_relief
                .tin()
                .unwrap()
                .object
                .as_ref()
                .unwrap()
                .patches()
                .patches_len(),
            1
        );
    }

    #[test]
    pub fn test_deserialize_tin_relief_basic() {
        let xml_document = "<dem:TINRelief>
          <dem:lod>2</dem:lod>
          <dem:tin>
            <gml:TriangulatedSurface gml:id=\"ground\">
              <gml:patches>
                <gml:Triangle>
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"Geo28109128\">
                      <gml:posList>513037.352708 5403284.890495 247.27 513034.424098 5403297.424959 247.19 513034.423981 5403287.429362 247.3 513037.352708 5403284.890495 247.27</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Triangle>
                <gml:Triangle>
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"Geo28109129\">
                      <gml:posList>513077.98546 5403284.940675 245.28 513073.615171 5403277.62549 244.056421 513080.474403 5403283.461339 245.28 513077.98546 5403284.940675 245.28</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Triangle>
              </gml:patches>
            </gml:TriangulatedSurface>
          </dem:tin>
        </dem:TINRelief>";

        let tin_relief = deserialize_tin_relief(xml_document.as_bytes()).expect("should work");

        assert_eq!(tin_relief.lod(), LevelOfDetail::Two);
        assert_eq!(
            tin_relief
                .tin()
                .unwrap()
                .object
                .as_ref()
                .unwrap()
                .patches()
                .patches_len(),
            2
        );
    }
}
