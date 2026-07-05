use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_thematic_surface, serialize_abstract_thematic_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, extract_xml_element_spans};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::{AsAbstractThematicSurface, ClosureSurface};

pub fn deserialize_closure_surface(xml_document: &[u8]) -> Result<ClosureSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_thematic_surface = deserialize_abstract_thematic_surface(xml_document, &spans)?;
    Ok(ClosureSurface::from_abstract_thematic_surface(
        abstract_thematic_surface,
    ))
}

pub fn serialize_closure_surface(
    closure_surface: &ClosureSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let parts = serialize_abstract_thematic_surface(
        closure_surface.abstract_thematic_surface(),
        formatting,
    )?;
    Ok(XmlNode::new(XmlElement::ClosureSurface, parts))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::building::{AbstractBuilding, AsAbstractBuildingMut};
    use ecitygml_core::model::construction::{
        AbstractConstruction, ConstructionSurfaceKind, WallSurface,
    };
    use ecitygml_core::model::core::{
        AbstractCityObject, AbstractFeature, AbstractFeatureWithLifespan, AbstractOccupiedSpace,
        AbstractPhysicalSpace, AbstractSpace, AsAbstractCityObject, AsAbstractFeature,
        AsAbstractFeatureMut, AsAbstractSpace, SpaceBoundaryKind, ThematicSurfaceKind,
    };
    use egml::model::base::Id;
    use egml::model::basic::Code;

    #[test]
    fn test_deserialize_empty_closure_surface() {
        let xml_document =
            b"<ClosureSurface gml:id=\"fme-gen-f460209b-63b5-4ff8-92d5-cec932987265\"/>";

        let closure_surface = deserialize_closure_surface(xml_document).expect("should work");

        assert_eq!(
            closure_surface.id(),
            &Id::try_from("fme-gen-f460209b-63b5-4ff8-92d5-cec932987265").expect("should work")
        );
    }
}
