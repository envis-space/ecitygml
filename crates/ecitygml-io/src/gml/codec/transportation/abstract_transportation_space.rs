use crate::Error;
use crate::gml::codec::core::deserialize_abstract_unoccupied_space;
use crate::gml::codec::transportation::auxiliary_traffic_space_property::deserialize_auxiliary_traffic_space_property;
use crate::gml::codec::transportation::marking_property::deserialize_marking_property;
use crate::gml::codec::transportation::traffic_direction_value::GmlTrafficDirectionValue;
use crate::gml::codec::transportation::traffic_space_property::deserialize_traffic_space_property;
use crate::gml::util::{XmlElement, XmlElementSpans, collect_children};
use ecitygml_core::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpaceMut,
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

    let mut abstract_transportation_space =
        AbstractTransportationSpace::new(abstract_unoccupied_space);
    abstract_transportation_space.markings =
        markings_result.expect("rayon::scope guarantees all spawns complete")?;
    abstract_transportation_space.traffic_spaces =
        traffic_spaces_result.expect("rayon::scope guarantees all spawns complete")?;
    abstract_transportation_space.auxiliary_traffic_spaces =
        auxiliary_traffic_spaces_result.expect("rayon::scope guarantees all spawns complete")?;
    abstract_transportation_space.set_traffic_direction(parsed.traffic_direction.map(|x| x.into()));

    Ok(abstract_transportation_space)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractTransportationSpace {
    #[serde(rename = "trafficDirection", default)]
    pub traffic_direction: Option<GmlTrafficDirectionValue>,
}
