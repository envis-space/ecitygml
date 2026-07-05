use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_filling_surface, serialize_abstract_filling_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, extract_xml_element_spans};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::{AsAbstractFillingSurface, WindowSurface};
use serde::{Deserialize, Serialize};

pub fn deserialize_window_surface(xml_document: &[u8]) -> Result<WindowSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_filling_surface = deserialize_abstract_filling_surface(xml_document, &spans)?;
    let window_surface = WindowSurface::from_abstract_filling_surface(abstract_filling_surface);

    Ok(window_surface)
}

pub fn serialize_window_surface(
    window_surface: &WindowSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let xml_node_parts =
        serialize_abstract_filling_surface(window_surface.abstract_filling_surface(), formatting)?;

    Ok(XmlNode::new(XmlElement::WindowSurface, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlWindowSurface {}

impl From<&WindowSurface> for GmlWindowSurface {
    fn from(_item: &WindowSurface) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{AsAbstractFeature, AsAbstractThematicSurface};
    use egml::model::base::Id;
    use itertools::Itertools;
    use quick_xml::{DeError, de};

    #[test]
    fn test_serialize_round_trip() {
        let xml_document = b"
            <con:WindowSurface gml:id=\"GML_d0f329f3-5b05-428d-87c3-945b3868337f\">
              <gml:name>Window West</gml:name>
              <lod3MultiSurface>
                <gml:MultiSurface>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58821_1939_612838_272028\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58821_1939_612838_272028_0\">
                          <gml:pos>457843.79 5439083.12 112.8</gml:pos>
                          <gml:pos>457845.79 5439083.12 112.8</gml:pos>
                          <gml:pos>457845.79 5439083.12 114.0</gml:pos>
                          <gml:pos>457843.79 5439083.12 114.0</gml:pos>
                          <gml:pos>457843.79 5439083.12 112.8</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                </gml:MultiSurface>
              </lod3MultiSurface>
            </con:WindowSurface>";

        let window_surface = deserialize_window_surface(xml_document).unwrap();
        let xml_output = serialize_window_surface(&window_surface, Formatting::default())
            .unwrap()
            .to_string(Formatting::default())
            .unwrap();

        assert!(xml_output.contains("con:WindowSurface"));
        assert!(xml_output.contains("gml:id=\"GML_d0f329f3-5b05-428d-87c3-945b3868337f\""));
        assert!(xml_output.contains("gml:name"));
        assert!(xml_output.contains("Window West"));
        assert!(xml_output.contains("lod3MultiSurface"));
        assert!(xml_output.contains("gml:MultiSurface"));
        assert!(xml_output.contains("gml:Polygon"));
    }

    #[test]
    fn test_serialize_with_generic_attributes() {
        let xml_document = b"
            <con:WindowSurface gml:id=\"test-window-id\">
              <genericAttribute>
                <gen:StringAttribute>
                  <gen:name>material</gen:name>
                  <gen:value>glass</gen:value>
                </gen:StringAttribute>
              </genericAttribute>
              <lod2MultiSurface>
                <gml:MultiSurface>
                  <gml:surfaceMember>
                    <gml:Polygon>
                      <gml:exterior>
                        <gml:LinearRing>
                          <gml:pos>0.0 0.0 0.0</gml:pos>
                          <gml:pos>1.0 0.0 0.0</gml:pos>
                          <gml:pos>1.0 1.0 0.0</gml:pos>
                          <gml:pos>0.0 0.0 0.0</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                </gml:MultiSurface>
              </lod2MultiSurface>
            </con:WindowSurface>";

        let window_surface = deserialize_window_surface(xml_document).unwrap();
        let xml_output = serialize_window_surface(&window_surface, Formatting::default())
            .unwrap()
            .to_string(Formatting::default())
            .unwrap();

        assert!(xml_output.contains("genericAttribute"));
        assert!(xml_output.contains("material"));
        assert!(xml_output.contains("glass"));
        assert!(xml_output.contains("lod2MultiSurface"));
    }

    #[test]
    fn test_deserialize_with_lod3_multi_surface() {
        let xml_document = b"
            <con:WindowSurface gml:id=\"GML_d0f329f3-5b05-428d-87c3-945b3868337f\">
              <gml:name>Window West</gml:name>
              <lod3MultiSurface>
                <gml:MultiSurface>
                  <gml:surfaceMember>
                    <gml:Polygon gml:id=\"PolyID58821_1939_612838_272028\">
                      <gml:exterior>
                        <gml:LinearRing gml:id=\"PolyID58821_1939_612838_272028_0\">
                          <gml:pos>457843.79 5439083.12 112.8</gml:pos>
                          <gml:pos>457845.79 5439083.12 112.8</gml:pos>
                          <gml:pos>457845.79 5439083.12 114.0</gml:pos>
                          <gml:pos>457843.79 5439083.12 114.0</gml:pos>
                          <gml:pos>457843.79 5439083.12 112.8</gml:pos>
                        </gml:LinearRing>
                      </gml:exterior>
                    </gml:Polygon>
                  </gml:surfaceMember>
                </gml:MultiSurface>
              </lod3MultiSurface>
            </con:WindowSurface>";

        let window_surface = deserialize_window_surface(xml_document).unwrap();

        assert_eq!(
            window_surface.id(),
            &Id::try_from("GML_d0f329f3-5b05-428d-87c3-945b3868337f").expect("should work")
        );
        assert_eq!(
            *window_surface.name().first().unwrap(),
            "Window West".to_string()
        );
        assert!(window_surface.lod3_multi_surface().is_some());
    }
}
