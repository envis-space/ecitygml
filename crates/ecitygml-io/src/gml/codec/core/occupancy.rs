use ecitygml_core::model::core::Occupancy;
use ecitygml_core::model::core::values::{IntervalValue, OccupantTypeValue};
use egml::io::codec::basic::GmlCode;
use egml::model::basic_types::Code;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlOccupancy {
    #[serde(rename = "numberOfOccupants")]
    pub number_of_occupants: i64,

    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<GmlCode>,

    #[serde(rename = "occupantType", skip_serializing_if = "Option::is_none")]
    pub occupant_type: Option<GmlCode>,
}

impl From<GmlOccupancy> for Occupancy {
    fn from(item: GmlOccupancy) -> Self {
        let mut occupancy = Self::new(item.number_of_occupants);
        occupancy.set_interval_opt(item.interval.map(Code::from).map(IntervalValue::from));
        occupancy.set_occupant_type_opt(
            item.occupant_type
                .map(Code::from)
                .map(OccupantTypeValue::from),
        );
        occupancy
    }
}

impl From<&Occupancy> for GmlOccupancy {
    fn from(item: &Occupancy) -> Self {
        Self {
            number_of_occupants: item.number_of_occupants(),
            interval: item.interval().map(|value| value.code().into()),
            occupant_type: item.occupant_type().map(|value| value.code().into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::{de, se};

    #[test]
    fn test_deserialize_occupancy() {
        let xml_document = b"<Occupancy>
    <numberOfOccupants>123</numberOfOccupants>
    <interval>myInterval</interval>
    <occupantType>myOccupantType</occupantType>
</Occupancy>";

        let gml_occupancy: GmlOccupancy =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let occupancy = Occupancy::from(gml_occupancy);

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
    fn test_deserialize_occupancy_without_optional_fields() {
        let xml_document = b"<Occupancy>
    <numberOfOccupants>123</numberOfOccupants>
</Occupancy>";

        let gml_occupancy: GmlOccupancy =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let occupancy = Occupancy::from(gml_occupancy);

        assert_eq!(occupancy.number_of_occupants(), 123);
        assert!(occupancy.interval().is_none());
        assert!(occupancy.occupant_type().is_none());
    }

    #[test]
    fn test_serialize_occupancy() {
        let mut occupancy = Occupancy::new(123);
        occupancy.set_interval(IntervalValue::from(Code::new("myInterval")));
        occupancy.set_occupant_type(OccupantTypeValue::from(Code::new("myOccupantType")));

        let xml = se::to_string_with_root("Occupancy", &GmlOccupancy::from(&occupancy))
            .expect("should serialize");

        assert!(xml.contains("Occupancy"));
        assert!(xml.contains("numberOfOccupants"));
        assert!(xml.contains("123"));
        assert!(xml.contains("interval"));
        assert!(xml.contains("myInterval"));
        assert!(xml.contains("occupantType"));
        assert!(xml.contains("myOccupantType"));
    }

    #[test]
    fn test_round_trip_occupancy() {
        let xml_document = b"<Occupancy>
    <numberOfOccupants>123</numberOfOccupants>
    <interval>myInterval</interval>
    <occupantType>myOccupantType</occupantType>
</Occupancy>";

        let gml_occupancy: GmlOccupancy =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let occupancy = Occupancy::from(gml_occupancy);

        let xml = se::to_string_with_root("Occupancy", &GmlOccupancy::from(&occupancy))
            .expect("should serialize");

        let round_tripped_gml: GmlOccupancy = de::from_str(&xml).expect("should work");
        let round_tripped = Occupancy::from(round_tripped_gml);

        assert_eq!(occupancy, round_tripped);
    }
}
