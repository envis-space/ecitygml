use crate::Error;
use crate::gml::codec::appearance::surface_data_property::{
    deserialize_surface_data_property, serialize_surface_data_property,
};
use crate::gml::codec::core::{deserialize_abstract_appearance, serialize_abstract_appearance};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{
    XmlNode, XmlNodeContent, collect_children, extract_xml_element_spans, serialize_inner,
};
use crate::gml::write::Formatting;
use ecitygml_core::model::appearance::Appearance;
use ecitygml_core::model::core::AsAbstractAppearance;
use serde::{Deserialize, Serialize};

pub fn deserialize_appearance(xml_document: &[u8]) -> Result<Appearance, Error> {
    let spans = extract_xml_element_spans(xml_document)?;

    let mut abstract_appearance_result = None;
    let mut parsed_result = None;
    let mut surface_data_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_appearance_result = Some(deserialize_abstract_appearance(xml_document, &spans))
        });
        s.spawn(|_| {
            parsed_result = Some(
                quick_xml::de::from_reader::<_, GmlAppearance>(xml_document).map_err(Error::from),
            );
        });
        s.spawn(|_| {
            surface_data_result = Some(collect_children(
                xml_document,
                &spans,
                XmlElement::SurfaceDataProperty,
                deserialize_surface_data_property,
            ));
        });
    });

    let abstract_appearance =
        abstract_appearance_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let surface_data = surface_data_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut appearance = Appearance::from_abstract_appearance(abstract_appearance);
    appearance.set_theme(parsed.theme);
    appearance.set_surface_data(surface_data);

    Ok(appearance)
}

pub fn serialize_appearance(
    appearance: &Appearance,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts =
        serialize_abstract_appearance(appearance.abstract_appearance(), formatting)?;

    if let Some(raw) = serialize_inner(GmlAppearance::from(appearance), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    for prop in appearance.surface_data() {
        let node = serialize_surface_data_property(prop, formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Child(node));
    }

    Ok(XmlNode::new(XmlElement::Appearance, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAppearance {
    #[serde(
        rename(serialize = "app:theme", deserialize = "theme"),
        skip_serializing_if = "Option::is_none"
    )]
    pub theme: Option<String>,
}

impl From<&Appearance> for GmlAppearance {
    fn from(item: &Appearance) -> Self {
        Self {
            theme: item.theme().to_owned().map(String::from),
        }
    }
}
