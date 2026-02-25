use crate::Error;
use crate::gml::parser::relief::abstract_relief_component::parse_abstract_relief_component;
use ecitygml_core::model::relief::TinRelief;
use egml::io::primitives::GmlTriangulatedSurfaceProperty;
use egml::model::geometry::primitives::TriangulatedSurface;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn parse_tin_relief(xml_document: &[u8]) -> Result<TinRelief, Error> {
    let abstract_relief_component = parse_abstract_relief_component(xml_document)?;
    let gml_tin_relief: GmlTinRelief = de::from_reader(xml_document)?;

    let tin: TriangulatedSurface = gml_tin_relief.tin.content.try_into()?;

    Ok(TinRelief::new(abstract_relief_component, tin))
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlTinRelief {
    pub lod: u8,
    pub tin: GmlTriangulatedSurfaceProperty,
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
    fn test_parse_tin_relief() {
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

        let tin_relief = parse_tin_relief(xml_document.as_bytes()).expect("should work");

        assert_eq!(tin_relief.id(), &Id::try_from("abc").unwrap());
        assert_eq!(tin_relief.lod(), LevelOfDetail::Three);
        assert_eq!(tin_relief.tin().patches().patches_len(), 1);
    }

    #[test]
    pub fn test_parse_tin_relief_basic() {
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

        let tin_relief = parse_tin_relief(xml_document.as_bytes()).expect("should work");

        assert_eq!(tin_relief.lod(), LevelOfDetail::Two);
        assert_eq!(tin_relief.tin().patches().patches_len(), 2);
    }
}
