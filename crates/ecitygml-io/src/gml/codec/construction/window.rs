use crate::Error;
use crate::gml::codec::construction::abstract_filling_element::{
    deserialize_abstract_filling_element, serialize_abstract_filling_element,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::construction::{AsAbstractFillingElement, Window};
use egml::io::util::{Formatting, XmlNode, extract_xml_element_spans};

pub fn deserialize_window(xml_document: &[u8]) -> Result<Window, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_filling_element = deserialize_abstract_filling_element(xml_document, &spans)?;
    let window = Window::from_abstract_filling_element(abstract_filling_element);

    Ok(window)
}

pub fn serialize_window(window: &Window, formatting: Formatting) -> Result<XmlNode, Error> {
    let xml_node_parts =
        serialize_abstract_filling_element(window.abstract_filling_element(), formatting)?;

    Ok(XmlNode::new(CityGmlElement::Window.into(), xml_node_parts))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::codec::core::deserialize_abstract_thematic_surface;
    use ecitygml_core::model::construction::{
        AbstractConstructionSurfaceKind, AbstractFillingSurfaceKind, AsAbstractConstructionSurface,
        Door, DoorSurface, WindowSurface,
    };
    use ecitygml_core::model::core::{
        AbstractSpaceBoundaryKind, AbstractThematicSurfaceKind, AsAbstractCityObject,
        AsAbstractFeature, AsAbstractSpace, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_deserialize_basic_window() {
        let xml_document = b"<con:Window gml:id=\"GML_d38cf762-c29d-4491-88c9-bdc89e141978\">
              <gml:name>Window name</gml:name>
              <gen:genericAttribute>
                <gen:IntAttribute>
                  <gen:name>cleaning_number</gen:name>
                  <gen:value>10</gen:value>
                </gen:IntAttribute>
              </gen:genericAttribute>
              </con:Window>";

        let window = deserialize_window(xml_document).expect("should work");

        assert_eq!(
            window.feature_id(),
            &Id::try_from("GML_d38cf762-c29d-4491-88c9-bdc89e141978").expect("should work")
        );
        //assert!(window.lod3_multi_surface().is_some());
        assert_eq!(window.generic_attributes().len(), 1);
    }
}
