use crate::Error;
use crate::gml::codec::core::thematic_surface_kind::{
    deserialize_thematic_surface_kind, serialize_thematic_surface_kind,
};
use crate::gml::codec::relief::{
    deserialize_relief_component_kind, deserialize_relief_feature, serialize_relief_component_kind,
    serialize_relief_feature,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::SpaceBoundaryKind;

pub fn deserialize_space_boundary_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<Option<SpaceBoundaryKind>, Error> {
    if let Some(x) = deserialize_thematic_surface_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    if let Some(span) = spans.first(XmlElement::ReliefFeature) {
        let relief_feature = deserialize_relief_feature(&xml_document[span.start..span.end])?;
        return Ok(Some(relief_feature.into()));
    }

    if let Some(x) = deserialize_relief_component_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_space_boundary_kind(
    space_boundary_kind: &SpaceBoundaryKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match space_boundary_kind {
        SpaceBoundaryKind::ThematicSurfaceKind(x) => serialize_thematic_surface_kind(x, formatting),
        SpaceBoundaryKind::ReliefFeature(x) => serialize_relief_feature(x, formatting),
        SpaceBoundaryKind::ReliefComponentKind(x) => serialize_relief_component_kind(x, formatting),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::util::extract_xml_element_spans;
    use ecitygml_core::model::core::AsAbstractFeature;
    use egml::model::base::Id;
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
