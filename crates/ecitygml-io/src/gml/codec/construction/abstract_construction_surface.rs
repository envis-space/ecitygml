use crate::Error;
use crate::gml::codec::construction::filling_surface_property::{
    deserialize_filling_surface_property, serialize_filling_surface_property,
};
use crate::gml::codec::core::{
    deserialize_abstract_thematic_surface, serialize_abstract_thematic_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{
    XmlElementSpans, XmlNodeContent, XmlNodeParts, collect_children, serialize_inner,
};
use crate::gml::write::Formatting;
use ecitygml_core::model::construction::{
    AbstractConstructionSurface, AsAbstractConstructionSurface, AsAbstractConstructionSurfaceMut,
};
use ecitygml_core::model::core::AsAbstractThematicSurface;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_construction_surface(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractConstructionSurface, Error> {
    let mut abstract_thematic_surface_result = None;
    let mut filling_surfaces_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_thematic_surface_result =
                Some(deserialize_abstract_thematic_surface(xml_document, spans));
        });
        s.spawn(|_| {
            filling_surfaces_result = Some(collect_children(
                xml_document,
                spans,
                XmlElement::FillingSurfaceProperty,
                deserialize_filling_surface_property,
            ));
        });
    });

    let abstract_thematic_surface =
        abstract_thematic_surface_result.expect("rayon::scope guarantees all spawns complete")?;
    let filling_surfaces =
        filling_surfaces_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut abstract_construction_surface =
        AbstractConstructionSurface::from_abstract_thematic_surface(abstract_thematic_surface);
    abstract_construction_surface.set_filling_surfaces(filling_surfaces);

    Ok(abstract_construction_surface)
}

pub fn serialize_abstract_construction_surface(
    abstract_construction_surface: &AbstractConstructionSurface,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = serialize_abstract_thematic_surface(
        abstract_construction_surface.abstract_thematic_surface(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlAbstractConstructionSurface::from(abstract_construction_surface),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    xml_node_parts.content.extend(
        abstract_construction_surface
            .filling_surfaces()
            .iter()
            .map(|x| serialize_filling_surface_property(x, formatting).map(XmlNodeContent::from))
            .collect::<Result<Vec<_>, _>>()?,
    );

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractConstructionSurface {}

impl From<&AbstractConstructionSurface> for GmlAbstractConstructionSurface {
    fn from(_item: &AbstractConstructionSurface) -> Self {
        Self {}
    }
}
