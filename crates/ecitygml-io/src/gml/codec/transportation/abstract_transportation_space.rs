use crate::Error;
use crate::gml::codec::core::{
    GmlOccupancyProperty, deserialize_abstract_unoccupied_space,
    serialize_abstract_unoccupied_space,
};
use crate::gml::codec::transportation::auxiliary_traffic_space_property::{
    deserialize_auxiliary_traffic_space_property, serialize_auxiliary_traffic_space_property,
};
use crate::gml::codec::transportation::hole_property::{
    deserialize_hole_property, serialize_hole_property,
};
use crate::gml::codec::transportation::marking_property::{
    deserialize_marking_property, serialize_marking_property,
};
use crate::gml::codec::transportation::traffic_direction_value::GmlTrafficDirectionValue;
use crate::gml::codec::transportation::traffic_space_property::{
    deserialize_traffic_space_property, serialize_traffic_space_property,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::core::AsAbstractUnoccupiedSpace;
use ecitygml_core::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, AsAbstractTransportationSpaceMut,
};
use egml::io::util::collect_children;
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_transportation_space(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractTransportationSpace, Error> {
    let mut abstract_unoccupied_space_result = None;
    let mut parsed_result = None;
    let mut traffic_spaces_result = None;
    let mut auxiliary_traffic_spaces_result = None;
    let mut markings_result = None;
    let mut holes_result = None;

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
            traffic_spaces_result = Some(collect_children(
                xml_document,
                spans,
                CityGmlElement::TrafficSpaceProperty.into(),
                deserialize_traffic_space_property,
            ));
        });
        s.spawn(|_| {
            auxiliary_traffic_spaces_result = Some(collect_children(
                xml_document,
                spans,
                CityGmlElement::AuxiliaryTrafficSpaceProperty.into(),
                deserialize_auxiliary_traffic_space_property,
            ));
        });
        s.spawn(|_| {
            markings_result = Some(collect_children(
                xml_document,
                spans,
                CityGmlElement::MarkingProperty.into(),
                deserialize_marking_property,
            ));
        });
        s.spawn(|_| {
            holes_result = Some(collect_children(
                xml_document,
                spans,
                CityGmlElement::HoleProperty.into(),
                deserialize_hole_property,
            ));
        });
    });

    let abstract_unoccupied_space =
        abstract_unoccupied_space_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let traffic_spaces =
        traffic_spaces_result.expect("rayon::scope guarantees all spawns complete")?;
    let auxiliary_traffic_spaces =
        auxiliary_traffic_spaces_result.expect("rayon::scope guarantees all spawns complete")?;
    let markings = markings_result.expect("rayon::scope guarantees all spawns complete")?;
    let holes = holes_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut abstract_transportation_space =
        AbstractTransportationSpace::from_abstract_unoccupied_space(abstract_unoccupied_space);
    abstract_transportation_space
        .set_traffic_direction_opt(parsed.traffic_direction.map(Into::into));
    abstract_transportation_space
        .set_occupancies(parsed.occupancies.into_iter().map(Into::into).collect());
    abstract_transportation_space.set_traffic_spaces(traffic_spaces);
    abstract_transportation_space.set_auxiliary_traffic_spaces(auxiliary_traffic_spaces);
    abstract_transportation_space.set_markings(markings);
    abstract_transportation_space.set_holes(holes);

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

    for hole_property in abstract_transportation_space.holes() {
        let node = serialize_hole_property(hole_property, formatting)?;
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

    #[serde(
        rename(serialize = "tran:occupancy", deserialize = "occupancy"),
        default
    )]
    pub occupancies: Vec<GmlOccupancyProperty>,
}

impl From<&AbstractTransportationSpace> for GmlAbstractTransportationSpace {
    fn from(item: &AbstractTransportationSpace) -> Self {
        Self {
            traffic_direction: item.traffic_direction().as_ref().map(Into::into),
            occupancies: item
                .occupancies()
                .iter()
                .map(GmlOccupancyProperty::from)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::transportation::AsAbstractTransportationSpace;
    use egml::io::util::extract_xml_element_spans;

    #[test]
    fn test_deserialize_abstract_transportation_space_with_occupancy() {
        let xml_document = b"\
<tran:Road gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <tran:occupancy>
        <Occupancy>
            <numberOfOccupants>123</numberOfOccupants>
            <interval>myInterval</interval>
            <occupantType>myOccupantType</occupantType>
        </Occupancy>
    </tran:occupancy>
</tran:Road>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_transportation_space =
            deserialize_abstract_transportation_space(xml_document, &spans).expect("should work");

        assert_eq!(abstract_transportation_space.occupancies().len(), 1);

        let occupancy = &abstract_transportation_space.occupancies()[0];
        assert_eq!(occupancy.number_of_occupants(), 123);
        assert_eq!(
            occupancy.interval().expect("must exist").code().value(),
            "myInterval"
        );
        assert_eq!(
            occupancy
                .occupant_type()
                .expect("must exist")
                .code()
                .value(),
            "myOccupantType"
        );
    }

    #[test]
    fn test_serialize_abstract_transportation_space_with_occupancy() {
        let xml_document = b"\
<tran:Road gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <tran:occupancy>
        <Occupancy>
            <numberOfOccupants>123</numberOfOccupants>
            <interval>myInterval</interval>
            <occupantType>myOccupantType</occupantType>
        </Occupancy>
    </tran:occupancy>
</tran:Road>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_transportation_space =
            deserialize_abstract_transportation_space(xml_document, &spans).expect("should work");

        let xml_node_parts = serialize_abstract_transportation_space(
            &abstract_transportation_space,
            Formatting::Compact,
        )
        .expect("should serialize");
        let xml = egml::io::util::XmlNode::new("tran:Road", xml_node_parts)
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        assert!(xml.contains("tran:occupancy"));
        assert!(xml.contains("Occupancy"));
        assert!(xml.contains("numberOfOccupants"));
        assert!(xml.contains("123"));
        assert!(xml.contains("myInterval"));
        assert!(xml.contains("myOccupantType"));
    }

    #[test]
    fn test_round_trip_abstract_transportation_space_with_occupancy() {
        let xml_document = b"\
<tran:Road gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <tran:occupancy>
        <Occupancy>
            <numberOfOccupants>123</numberOfOccupants>
            <interval>myInterval</interval>
            <occupantType>myOccupantType</occupantType>
        </Occupancy>
    </tran:occupancy>
</tran:Road>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_transportation_space =
            deserialize_abstract_transportation_space(xml_document, &spans).expect("should work");

        let xml_node_parts = serialize_abstract_transportation_space(
            &abstract_transportation_space,
            Formatting::Compact,
        )
        .expect("should serialize");
        let xml = egml::io::util::XmlNode::new("tran:Road", xml_node_parts)
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        let round_tripped_spans = extract_xml_element_spans(xml.as_bytes()).expect("should work");
        let round_tripped =
            deserialize_abstract_transportation_space(xml.as_bytes(), &round_tripped_spans)
                .expect("should work");

        assert_eq!(
            abstract_transportation_space.occupancies(),
            round_tripped.occupancies()
        );
    }
}
