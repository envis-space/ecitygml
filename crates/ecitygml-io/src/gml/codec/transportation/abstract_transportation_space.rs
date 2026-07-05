use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_unoccupied_space, serialize_abstract_unoccupied_space,
};
use crate::gml::codec::transportation::auxiliary_traffic_space_property::{
    deserialize_auxiliary_traffic_space_property, serialize_auxiliary_traffic_space_property,
};
use crate::gml::codec::transportation::marking_property::{
    deserialize_marking_property, serialize_marking_property,
};
use crate::gml::codec::transportation::traffic_direction_value::GmlTrafficDirectionValue;
use crate::gml::codec::transportation::traffic_space_property::{
    deserialize_traffic_space_property, serialize_traffic_space_property,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{
    XmlElementSpans, XmlNodeContent, XmlNodeParts, collect_children, serialize_inner,
};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::AsAbstractUnoccupiedSpace;
use ecitygml_core::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, AsAbstractTransportationSpaceMut,
};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_transportation_space(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractTransportationSpace, Error> {
    let mut abstract_unoccupied_space_result = None;
    let mut parsed_result = None;
    let mut markings_result = None;
    let mut traffic_spaces_result = None;
    let mut auxiliary_traffic_spaces_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_unoccupied_space_result =
                Some(deserialize_abstract_unoccupied_space(xml_document, spans));
        });
        s.spawn(|_| {
            parsed_result = Some(
                de::from_reader::<_, GmlAbstractTransportationSpace>(xml_document)
                    .map_err(Error::from),
            );
        });
        s.spawn(|_| {
            markings_result = Some(collect_children(
                xml_document,
                spans,
                XmlElement::MarkingProperty,
                deserialize_marking_property,
            ));
        });
        s.spawn(|_| {
            traffic_spaces_result = Some(collect_children(
                xml_document,
                spans,
                XmlElement::TrafficSpaceProperty,
                deserialize_traffic_space_property,
            ));
        });
        s.spawn(|_| {
            auxiliary_traffic_spaces_result = Some(collect_children(
                xml_document,
                spans,
                XmlElement::AuxiliaryTrafficSpaceProperty,
                deserialize_auxiliary_traffic_space_property,
            ));
        });
    });

    let abstract_unoccupied_space =
        abstract_unoccupied_space_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let markings = markings_result.expect("rayon::scope guarantees all spawns complete")?;
    let traffic_spaces =
        traffic_spaces_result.expect("rayon::scope guarantees all spawns complete")?;
    let auxiliary_traffic_spaces =
        auxiliary_traffic_spaces_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut abstract_transportation_space =
        AbstractTransportationSpace::from_abstract_unoccupied_space(abstract_unoccupied_space);
    abstract_transportation_space.set_traffic_direction(parsed.traffic_direction.map(Into::into));
    abstract_transportation_space.set_markings(markings);
    abstract_transportation_space.set_traffic_spaces(traffic_spaces);
    abstract_transportation_space.set_auxiliary_traffic_spaces(auxiliary_traffic_spaces);

    Ok(abstract_transportation_space)
}

pub fn serialize_abstract_transportation_space(
    abstract_transportation_space: &AbstractTransportationSpace,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = serialize_abstract_unoccupied_space(
        abstract_transportation_space.abstract_unoccupied_space(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlAbstractTransportationSpace::from(abstract_transportation_space),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    for traffic_space_property in abstract_transportation_space.traffic_spaces() {
        let node = serialize_traffic_space_property(traffic_space_property, formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Child(node));
    }

    for auxiliary_traffic_space_property in abstract_transportation_space.auxiliary_traffic_spaces()
    {
        let node = serialize_auxiliary_traffic_space_property(
            auxiliary_traffic_space_property,
            formatting,
        )?;
        xml_node_parts.content.push(XmlNodeContent::Child(node));
    }

    for marking_property in abstract_transportation_space.markings() {
        let node = serialize_marking_property(marking_property, formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Child(node));
    }

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractTransportationSpace {
    #[serde(
        rename(serialize = "tran:trafficDirection", deserialize = "trafficDirection"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_direction: Option<GmlTrafficDirectionValue>,
}

impl From<&AbstractTransportationSpace> for GmlAbstractTransportationSpace {
    fn from(item: &AbstractTransportationSpace) -> Self {
        Self {
            traffic_direction: item.traffic_direction().as_ref().map(Into::into),
        }
    }
}
