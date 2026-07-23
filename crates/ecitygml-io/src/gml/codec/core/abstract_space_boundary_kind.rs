use crate::Error;
use crate::gml::codec::core::abstract_thematic_surface_kind::{
    deserialize_abstract_thematic_surface_kind, serialize_abstract_thematic_surface_kind,
};
use crate::gml::codec::relief::{
    deserialize_abstract_relief_component_kind, deserialize_relief_feature,
    serialize_abstract_relief_component_kind, serialize_relief_feature,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::AbstractSpaceBoundaryKind;
use egml::io::util::{Formatting, XmlElementSpans, XmlNode};

pub fn deserialize_abstract_space_boundary_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<Option<AbstractSpaceBoundaryKind>, Error> {
    if let Some(x) = deserialize_abstract_thematic_surface_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    if let Some(span) = spans.first(CityGmlElement::ReliefFeature.into()) {
        let relief_feature = deserialize_relief_feature(&xml_document[span.start..span.end])?;
        return Ok(Some(relief_feature.into()));
    }

    if let Some(x) = deserialize_abstract_relief_component_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_space_boundary_kind(
    abstract_space_boundary_kind: &AbstractSpaceBoundaryKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_space_boundary_kind {
        AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(x) => {
            serialize_abstract_thematic_surface_kind(x, formatting)
        }
        AbstractSpaceBoundaryKind::ReliefFeature(x) => serialize_relief_feature(x, formatting),
        AbstractSpaceBoundaryKind::AbstractReliefComponentKind(x) => {
            serialize_abstract_relief_component_kind(x, formatting)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::AsAbstractFeature;
    use egml::io::util::extract_xml_element_spans;
    use egml::model::base::{AsAbstractGml, Id};
    use egml::model::basic_types::Code;

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
        let abstract_space_boundary_kind =
            deserialize_abstract_space_boundary_kind(xml_document, &spans)
                .expect("should work")
                .expect("should parse one space boundary kind");

        assert_eq!(
            abstract_space_boundary_kind.feature_id(),
            &Id::try_from("GML_5856d7ad-5e34-498a-817b-9544bfbb1475").expect("should work")
        );

        assert_eq!(
            abstract_space_boundary_kind
                .names()
                .first()
                .expect("should work"),
            &Code::new("Outer Wall 1 (West)")
        );
    }
}
