use crate::Error;
use crate::gml::codec::core::{
    GmlOccupancyProperty, deserialize_abstract_unoccupied_space,
    serialize_abstract_unoccupied_space,
};
use crate::gml::codec::transportation::clearance_space_property::{
    deserialize_clearance_space_property, serialize_clearance_space_property,
};
use crate::gml::codec::transportation::granularity_value::GmlGranularityValue;
use crate::gml::codec::transportation::traffic_direction_value::GmlTrafficDirectionValue;
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::core::AsAbstractUnoccupiedSpace;
use ecitygml_core::model::transportation::values::{
    TrafficSpaceClassValue, TrafficSpaceFunctionValue, TrafficSpaceUsageValue,
};
use ecitygml_core::model::transportation::{TrafficSpace, TrafficSpaceReference};
use egml::io::codec::base::GmlReference;
use egml::io::codec::basic::GmlCode;
use egml::io::util::collect_children;
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::base::Reference;
use egml::model::basic_types::Code;
use serde::{Deserialize, Serialize};

pub fn deserialize_traffic_space(xml_document: &[u8]) -> Result<TrafficSpace, Error> {
    let spans = extract_xml_element_spans(xml_document)?;

    let mut abstract_unoccupied_space_result = None;
    let mut parsed_result = None;
    let mut clearance_spaces_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_unoccupied_space_result =
                Some(deserialize_abstract_unoccupied_space(xml_document, &spans));
        });
        s.spawn(|_| {
            parsed_result = Some(
                quick_xml::de::from_reader::<_, GmlTrafficSpace>(xml_document).map_err(Error::from),
            );
        });
        s.spawn(|_| {
            clearance_spaces_result = Some(collect_children(
                xml_document,
                &spans,
                CityGmlElement::ClearanceSpaceProperty.into(),
                deserialize_clearance_space_property,
            ));
        });
    });

    let abstract_unoccupied_space =
        abstract_unoccupied_space_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let clearance_spaces =
        clearance_spaces_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut traffic_space = TrafficSpace::from_abstract_unoccupied_space(
        abstract_unoccupied_space,
        parsed.granularity.into(),
    );

    traffic_space.set_class_opt(
        parsed
            .class
            .map(Code::from)
            .map(TrafficSpaceClassValue::from),
    );
    traffic_space.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(TrafficSpaceFunctionValue::from)
            .collect(),
    );
    traffic_space.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(TrafficSpaceUsageValue::from)
            .collect(),
    );
    traffic_space.set_traffic_direction_opt(parsed.traffic_direction.map(Into::into));
    traffic_space.set_occupancies(parsed.occupancies.into_iter().map(Into::into).collect());
    traffic_space.set_predecessors(
        parsed
            .predecessors
            .into_iter()
            .map(Reference::try_from)
            .collect::<Result<Vec<Reference>, _>>()?
            .into_iter()
            .map(TrafficSpaceReference::from)
            .collect(),
    );
    traffic_space.set_successors(
        parsed
            .successors
            .into_iter()
            .map(Reference::try_from)
            .collect::<Result<Vec<Reference>, _>>()?
            .into_iter()
            .map(TrafficSpaceReference::from)
            .collect(),
    );

    traffic_space.set_clearance_spaces(clearance_spaces);

    Ok(traffic_space)
}

pub fn serialize_traffic_space(
    traffic_space: &TrafficSpace,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts =
        serialize_abstract_unoccupied_space(traffic_space.abstract_unoccupied_space(), formatting)?;

    if let Some(raw) = serialize_inner(GmlTrafficSpace::from(traffic_space), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    for prop in traffic_space.clearance_spaces() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_clearance_space_property(
                prop, formatting,
            )?));
    }

    Ok(XmlNode::new(
        CityGmlElement::TrafficSpace.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTrafficSpace {
    #[serde(
        rename(serialize = "tran:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "tran:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "tran:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,

    #[serde(
        rename(serialize = "tran:granularity", deserialize = "granularity"),
        default
    )]
    pub granularity: GmlGranularityValue,

    #[serde(
        rename(serialize = "tran:trafficDirection", deserialize = "trafficDirection"),
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_direction: Option<GmlTrafficDirectionValue>,

    #[serde(
        rename(serialize = "tran:occupancy", deserialize = "occupancy"),
        default
    )]
    pub occupancies: Vec<GmlOccupancyProperty>,

    #[serde(
        rename(serialize = "tran:predecessor", deserialize = "predecessor"),
        default
    )]
    pub predecessors: Vec<GmlReference>,

    #[serde(
        rename(serialize = "tran:successor", deserialize = "successor"),
        default
    )]
    pub successors: Vec<GmlReference>,
}

impl From<&TrafficSpace> for GmlTrafficSpace {
    fn from(item: &TrafficSpace) -> Self {
        Self {
            class: item
                .class()
                .map(TrafficSpaceClassValue::code)
                .map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(TrafficSpaceFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(TrafficSpaceUsageValue::code)
                .map(Into::into)
                .collect(),
            granularity: item.granularity().into(),
            traffic_direction: item.traffic_direction().as_ref().map(Into::into),
            occupancies: item
                .occupancies()
                .iter()
                .map(GmlOccupancyProperty::from)
                .collect(),
            predecessors: item
                .predecessors()
                .iter()
                .map(Reference::from)
                .map(|reference| GmlReference::from(&reference))
                .collect(),
            successors: item
                .successors()
                .iter()
                .map(Reference::from)
                .map(|reference| GmlReference::from(&reference))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::enums::SpaceType;
    use ecitygml_core::model::core::{
        AbstractSpaceBoundaryKind, AbstractThematicSurfaceKind, AsAbstractCityObject,
        AsAbstractFeature, AsAbstractSpace,
    };
    use ecitygml_core::model::transportation::{
        GranularityValue, TrafficArea, TrafficDirectionValue,
    };
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_basic_section() {
        let xml_document =
            b"<tran:TrafficSpace gml:id=\"UUID_6e4de408-1e54-3869-b7ce-1be3f2261421\">
              <genericAttribute>
                <gen:StringAttribute>
                  <gen:name>opendrive_lane_type</gen:name>
                  <gen:value>DRIVING</gen:value>
                </gen:StringAttribute>
              </genericAttribute>
              <spaceType>open</spaceType>
              <boundary>
                <tran:TrafficArea gml:id=\"UUID_dc110e80-dadc-3c87-b864-2854cc0cb39a\">
                  <gml:name>Lane</gml:name>
                  <genericAttribute>
                    <gen:IntAttribute>
                      <gen:name>identifier_laneId</gen:name>
                      <gen:value>-1</gen:value>
                    </gen:IntAttribute>
                  </genericAttribute>
                  <tran:function>1</tran:function>
                  <tran:usage>2</tran:usage>
                </tran:TrafficArea>
              </boundary>
              <lod2MultiCurve>
                <gml:MultiCurve>
                  <gml:curveMember>
                    <gml:LineString>
                      <gml:posList srsDimension=\"3\">-52.6446227593087 1.5877425640718097 0.0 -47.64466181585868 1.6075052555478968 0.0 -42.64470087240866 1.627267947023984 0.0 -37.64473992895864 1.6470306385000706 0.0 -32.64477898550862 1.6667933299761577 0.0 -27.6448180420586 1.6865560214522448 0.0 -22.64485709860858 1.706318712928332 0.0 -17.644896155158555 1.7260814044044186 0.0 -15.898072287268029 1.7329858465699939 0.0</gml:posList>
                    </gml:LineString>
                  </gml:curveMember>
                </gml:MultiCurve>
              </lod2MultiCurve>
              <tran:function>1</tran:function>
              <tran:usage>2</tran:usage>
              <tran:granularity>lane</tran:granularity>
              <tran:trafficDirection>forwards</tran:trafficDirection>
              <tran:successor xlink:href=\"#UUID_144a6807-5844-32b2-bb34-8b2671b1afaa\"/>
            </tran:TrafficSpace>";

        let traffic_space = deserialize_traffic_space(xml_document).expect("should work");

        assert_eq!(
            traffic_space.feature_id(),
            &Id::try_from("UUID_6e4de408-1e54-3869-b7ce-1be3f2261421").expect("should work")
        );
        assert!(traffic_space.lod2_multi_surface().is_none());
        assert_eq!(traffic_space.generic_attributes().len(), 1);
        assert_eq!(traffic_space.space_type(), Some(SpaceType::Open));
        assert_eq!(
            traffic_space.functions().first().unwrap().code().value(),
            "1"
        );
        assert_eq!(traffic_space.usages().first().unwrap().code().value(), "2");
        assert_eq!(traffic_space.granularity(), &GranularityValue::Lane);
        assert_eq!(
            traffic_space.traffic_direction().unwrap(),
            TrafficDirectionValue::Forwards
        );
        let traffic_areas: Vec<&TrafficArea> = traffic_space
            .boundaries()
            .iter()
            .flat_map(|x| x.object())
            .filter_map(|x| match x {
                AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(
                    AbstractThematicSurfaceKind::TrafficArea(x),
                ) => Some(x),
                _ => None,
            })
            .collect();
        assert_eq!(traffic_areas.len(), 1);

        let traffic_area = traffic_areas.first().unwrap();
        assert_eq!(
            traffic_area.feature_id(),
            &Id::try_from("UUID_dc110e80-dadc-3c87-b864-2854cc0cb39a").expect("should work")
        );
        assert_eq!(traffic_area.generic_attributes().len(), 1);
        assert_eq!(
            traffic_area.functions().first().unwrap().code().value(),
            "1"
        );
        assert_eq!(traffic_area.usages().first().unwrap().code().value(), "2");
    }

    #[test]
    fn test_deserialize_traffic_space_with_occupancy() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <tran:granularity>lane</tran:granularity>
    <tran:occupancy>
        <Occupancy>
            <numberOfOccupants>123</numberOfOccupants>
            <interval>myInterval</interval>
            <occupantType>myOccupantType</occupantType>
        </Occupancy>
    </tran:occupancy>
</tran:TrafficSpace>";

        let traffic_space = deserialize_traffic_space(xml_document).expect("should work");

        assert_eq!(traffic_space.occupancies().len(), 1);

        let occupancy = &traffic_space.occupancies()[0];
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
    fn test_serialize_traffic_space_with_occupancy() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <tran:granularity>lane</tran:granularity>
    <tran:occupancy>
        <Occupancy>
            <numberOfOccupants>123</numberOfOccupants>
            <interval>myInterval</interval>
            <occupantType>myOccupantType</occupantType>
        </Occupancy>
    </tran:occupancy>
</tran:TrafficSpace>";

        let traffic_space = deserialize_traffic_space(xml_document).expect("should work");

        let xml = serialize_traffic_space(&traffic_space, Formatting::Compact)
            .expect("should serialize")
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
    fn test_round_trip_traffic_space_with_occupancy() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <tran:granularity>lane</tran:granularity>
    <tran:occupancy>
        <Occupancy>
            <numberOfOccupants>123</numberOfOccupants>
            <interval>myInterval</interval>
            <occupantType>myOccupantType</occupantType>
        </Occupancy>
    </tran:occupancy>
</tran:TrafficSpace>";

        let traffic_space = deserialize_traffic_space(xml_document).expect("should work");

        let xml = serialize_traffic_space(&traffic_space, Formatting::Compact)
            .expect("should serialize")
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        let round_tripped = deserialize_traffic_space(xml.as_bytes()).expect("should work");

        assert_eq!(traffic_space.occupancies(), round_tripped.occupancies());
    }

    #[test]
    fn test_round_trip_traffic_space_with_predecessor() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <tran:granularity>lane</tran:granularity>
    <tran:predecessor xlink:href=\"#UUID_ed2149e3-421a-3dcd-9727-54637db9d9e3\"/>
</tran:TrafficSpace>";

        let traffic_space = deserialize_traffic_space(xml_document).expect("should work");

        assert_eq!(traffic_space.predecessors().len(), 1);

        let xml = serialize_traffic_space(&traffic_space, Formatting::Compact)
            .expect("should serialize")
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        assert!(xml.contains("tran:predecessor"));
        assert!(xml.contains("UUID_ed2149e3-421a-3dcd-9727-54637db9d9e3"));

        let round_tripped = deserialize_traffic_space(xml.as_bytes()).expect("should work");

        assert_eq!(traffic_space.predecessors(), round_tripped.predecessors());
    }

    #[test]
    fn test_round_trip_traffic_space_with_successor() {
        let xml_document = b"\
<tran:TrafficSpace gml:id=\"UUID_ae81947d-a661-3678-a5ee-eed58b68694f\">
    <tran:granularity>lane</tran:granularity>
    <tran:successor xlink:href=\"#UUID_144a6807-5844-32b2-bb34-8b2671b1afaa\"/>
</tran:TrafficSpace>";

        let traffic_space = deserialize_traffic_space(xml_document).expect("should work");

        assert_eq!(traffic_space.successors().len(), 1);

        let xml = serialize_traffic_space(&traffic_space, Formatting::Compact)
            .expect("should serialize")
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        assert!(xml.contains("tran:successor"));
        assert!(xml.contains("UUID_144a6807-5844-32b2-bb34-8b2671b1afaa"));

        let round_tripped = deserialize_traffic_space(xml.as_bytes()).expect("should work");

        assert_eq!(traffic_space.successors(), round_tripped.successors());
    }
}
