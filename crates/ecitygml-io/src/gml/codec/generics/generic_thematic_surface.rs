use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_thematic_surface, serialize_abstract_thematic_surface,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::core::AsAbstractThematicSurface;
use ecitygml_core::model::generics::GenericThematicSurface;
use ecitygml_core::model::generics::values::{
    GenericThematicSurfaceClassValue, GenericThematicSurfaceFunctionValue,
    GenericThematicSurfaceUsageValue,
};
use egml::io::codec::basic::GmlCode;
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::basic_types::Code;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_generic_thematic_surface(
    xml_document: &[u8],
) -> Result<GenericThematicSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_thematic_surface_result, parsed_result) = rayon::join(
        || deserialize_abstract_thematic_surface(xml_document, &spans),
        || de::from_reader::<_, GmlGenericThematicSurface>(xml_document).map_err(Error::from),
    );
    let abstract_thematic_surface = abstract_thematic_surface_result?;
    let parsed = parsed_result?;

    let mut generic_thematic_surface =
        GenericThematicSurface::from_abstract_thematic_surface(abstract_thematic_surface);
    generic_thematic_surface.set_class_opt(
        parsed
            .class
            .map(Code::from)
            .map(GenericThematicSurfaceClassValue::from),
    );
    generic_thematic_surface.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(GenericThematicSurfaceFunctionValue::from)
            .collect(),
    );
    generic_thematic_surface.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(GenericThematicSurfaceUsageValue::from)
            .collect(),
    );

    Ok(generic_thematic_surface)
}

pub fn serialize_generic_thematic_surface(
    generic_thematic_surface: &GenericThematicSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = serialize_abstract_thematic_surface(
        generic_thematic_surface.abstract_thematic_surface(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlGenericThematicSurface::from(generic_thematic_surface),
        formatting,
    )? {
        parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(
        CityGmlElement::GenericThematicSurface.into(),
        parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlGenericThematicSurface {
    #[serde(
        rename(serialize = "gen:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "gen:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "gen:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,
}

impl From<&GenericThematicSurface> for GmlGenericThematicSurface {
    fn from(item: &GenericThematicSurface) -> Self {
        Self {
            class: item
                .class()
                .map(GenericThematicSurfaceClassValue::code)
                .map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(GenericThematicSurfaceFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(GenericThematicSurfaceUsageValue::code)
                .map(Into::into)
                .collect(),
        }
    }
}
