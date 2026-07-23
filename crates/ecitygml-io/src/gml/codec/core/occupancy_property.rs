use crate::gml::codec::core::occupancy::GmlOccupancy;
use ecitygml_core::model::core::Occupancy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlOccupancyProperty {
    #[serde(rename = "Occupancy")]
    pub occupancy: GmlOccupancy,
}

impl From<GmlOccupancyProperty> for Occupancy {
    fn from(item: GmlOccupancyProperty) -> Self {
        Self::from(item.occupancy)
    }
}

impl From<&Occupancy> for GmlOccupancyProperty {
    fn from(item: &Occupancy) -> Self {
        Self {
            occupancy: GmlOccupancy::from(item),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::values::{IntervalValue, OccupantTypeValue};
    use egml::model::basic_types::Code;
    use quick_xml::{de, se};

    #[test]
    fn test_deserialize_occupancy_property() {
        let xml_document = b"<occupancy>
    <Occupancy>
        <numberOfOccupants>123</numberOfOccupants>
        <interval>myInterval</interval>
        <occupantType>myOccupantType</occupantType>
    </Occupancy>
</occupancy>";

        let gml_occupancy_property: GmlOccupancyProperty =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let occupancy = Occupancy::from(gml_occupancy_property);

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
    fn test_serialize_occupancy_property() {
        let mut occupancy = Occupancy::new(123);
        occupancy.set_interval(IntervalValue::from(Code::new("myInterval")));
        occupancy.set_occupant_type(OccupantTypeValue::from(Code::new("myOccupantType")));

        let xml = se::to_string_with_root("occupancy", &GmlOccupancyProperty::from(&occupancy))
            .expect("should serialize");

        assert!(xml.contains("Occupancy"));
        assert!(xml.contains("numberOfOccupants"));
        assert!(xml.contains("123"));
        assert!(xml.contains("myInterval"));
        assert!(xml.contains("myOccupantType"));
    }

    #[test]
    fn test_round_trip_occupancy_property() {
        let xml_document = b"<occupancy>
    <Occupancy>
        <numberOfOccupants>123</numberOfOccupants>
        <interval>myInterval</interval>
        <occupantType>myOccupantType</occupantType>
    </Occupancy>
</occupancy>";

        let gml_occupancy_property: GmlOccupancyProperty =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let occupancy = Occupancy::from(gml_occupancy_property);

        let xml = se::to_string_with_root("occupancy", &GmlOccupancyProperty::from(&occupancy))
            .expect("should serialize");

        let round_tripped_gml: GmlOccupancyProperty = de::from_str(&xml).expect("should work");
        let round_tripped = Occupancy::from(round_tripped_gml);

        assert_eq!(occupancy, round_tripped);
    }
}
