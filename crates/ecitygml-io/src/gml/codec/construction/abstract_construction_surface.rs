use crate::Error;
use crate::gml::codec::construction::filling_surface_property::deserialize_filling_surface_property;
use crate::gml::codec::core::deserialize_abstract_thematic_surface;
use crate::gml::util::{XmlElement, XmlElementSpans, collect_children};
use ecitygml_core::model::construction::{
    AbstractConstructionSurface, AsAbstractConstructionSurfaceMut,
};

pub fn deserialize_abstract_construction_surface(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractConstructionSurface, Error> {
    let abstract_thematic_surface = deserialize_abstract_thematic_surface(xml_document, spans)?;
    let mut abstract_construction_surface =
        AbstractConstructionSurface::new(abstract_thematic_surface);

    let filling_surfaces = collect_children(
        xml_document,
        spans,
        XmlElement::FillingSurfaceProperty,
        deserialize_filling_surface_property,
    )?;
    abstract_construction_surface.set_filling_surfaces(filling_surfaces);

    Ok(abstract_construction_surface)
}
