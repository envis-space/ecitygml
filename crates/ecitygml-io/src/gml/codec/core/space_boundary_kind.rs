use crate::Error;
use crate::gml::codec::core::thematic_surface_kind::deserialize_thematic_surface_kind;
use crate::gml::codec::relief::deserialize_relief_feature;
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::core::SpaceBoundaryKind;

pub fn deserialize_space_boundary_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<SpaceBoundaryKind>, Error> {
    if let Some(x) = deserialize_thematic_surface_kind(xml_document, spans)? {
        return Ok(Some(SpaceBoundaryKind::ThematicSurfaceKind(x)));
    }

    if let Some(span) = spans.first(XmlElement::ReliefFeature) {
        let relief_feature = deserialize_relief_feature(&xml_document[span.start..span.end])?;
        return Ok(Some(SpaceBoundaryKind::ReliefFeature(relief_feature)));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::codec::building::deserialize_building;
    use crate::gml::codec::core::deserialize_abstract_thematic_surface;
    use crate::gml::util::extract_xml_element_spans;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_deserialize_basic_wall_surface_kind() {
        let xml_document = b"
        <boundary>
            <con:WallSurface gml:id=\"GML_5856d7ad-5e34-498a-817b-9544bfbb1475\">
              <gml:name>Outer Wall 1 (West)</gml:name>
              <lod2MultiSurface>
                <gml:MultiSurface>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID7350_878_759628_120742\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID7350_878_759628_120742_0\">
                          <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                          <gml:pos>457842.0 5439093.0 115.430940107676</gml:pos>
                          <gml:pos>457842.0 5439093.0 111.8</gml:pos>
                          <gml:pos>457842.0 5439083.0 111.8</gml:pos>
                          <gml:pos>457842.0 5439083.0 115.430940107676</gml:pos>
                          <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                </gml:MultiSurface>
              </lod2MultiSurface>
            </con:WallSurface>
        </boundary>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let space_boundary_kind = deserialize_space_boundary_kind(xml_document, &spans)
            .expect("should work")
            .expect("should parse one space boundary kind");

        assert_eq!(
            space_boundary_kind.id(),
            &Id::try_from("GML_5856d7ad-5e34-498a-817b-9544bfbb1475").expect("should work")
        );
        assert_eq!(
            space_boundary_kind.name().first().expect("should work"),
            "Outer Wall 1 (West)"
        );
    }
}
