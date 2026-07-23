use crate::Error;
use crate::gml::codec::core::{
    GmlOccupancyProperty, deserialize_abstract_occupied_space, serialize_abstract_occupied_space,
};
use crate::gml::util::CombinedCityGmlElement;
use chrono::NaiveDate;
use ecitygml_core::model::construction::{
    AbstractConstruction, AsAbstractConstruction, AsAbstractConstructionMut,
};
use ecitygml_core::model::core::AsAbstractOccupiedSpace;
use egml::io::util::{Formatting, XmlElementSpans, XmlNodeContent, XmlNodeParts, serialize_inner};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_construction(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
) -> Result<AbstractConstruction, Error> {
    let (abstract_occupied_space_result, parsed_result) = rayon::join(
        || deserialize_abstract_occupied_space(xml_document, spans),
        || de::from_reader::<_, GmlAbstractConstruction>(xml_document).map_err(Error::from),
    );
    let abstract_occupied_space = abstract_occupied_space_result?;
    let parsed = parsed_result?;
    let mut abstract_construction =
        AbstractConstruction::from_abstract_occupied_space(abstract_occupied_space);

    abstract_construction.set_date_of_construction(parsed.date_of_construction);
    abstract_construction.set_date_of_demolition(parsed.date_of_demolition);
    abstract_construction.set_occupancies(parsed.occupancies.into_iter().map(Into::into).collect());

    Ok(abstract_construction)
}

pub fn serialize_abstract_construction(
    abstract_construction: &AbstractConstruction,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = serialize_abstract_occupied_space(
        abstract_construction.abstract_occupied_space(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlAbstractConstruction::from(abstract_construction),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractConstruction {
    #[serde(
        rename(
            serialize = "con:dateOfConstruction",
            deserialize = "dateOfConstruction"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub date_of_construction: Option<NaiveDate>,

    #[serde(
        rename(serialize = "con:dateOfDemolition", deserialize = "dateOfDemolition"),
        skip_serializing_if = "Option::is_none"
    )]
    pub date_of_demolition: Option<NaiveDate>,

    #[serde(
        rename(serialize = "con:occupancy", deserialize = "occupancy"),
        default
    )]
    pub occupancies: Vec<GmlOccupancyProperty>,
}

impl From<&AbstractConstruction> for GmlAbstractConstruction {
    fn from(item: &AbstractConstruction) -> Self {
        Self {
            date_of_construction: item.date_of_construction().copied(),
            date_of_demolition: item.date_of_demolition().copied(),
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
    use crate::gml::codec::building::deserialize_building;
    use crate::gml::codec::core::deserialize_abstract_thematic_surface;
    use chrono::{Datelike, Timelike};
    use ecitygml_core::model::construction::AsAbstractConstruction;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractFeatureWithLifespan,
        AsAbstractThematicSurface,
    };
    use egml::io::util::extract_xml_element_spans;
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_deserialize_basic_abstract_construction() {
        let xml_document = b"
<bldg:Building gml:id=\"DEBY_LOD2_4959457\">
      <creationDate>2025-03-02T15:16:17+01:00</creationDate>
      <externalReference>
        <ExternalReference>
          <targetResource>DEBYvAAAAABTMIyA</targetResource>
          <informationSystem>http://repository.gdi-de.org/schemas/adv/citygml/fdv/art.htm#_9100</informationSystem>
        </ExternalReference>
      </externalReference>
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>DatenquelleBodenhoehe</gen:name>
          <gen:value>1100</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <con:height>
        <con:Height>
          <con:highReference>highestRoofEdge</con:highReference>
          <con:lowReference>lowestGroundPoint</con:lowReference>
          <con:status>measured</con:status>
          <con:value uom=\"urn:adv:uom:m\">19.408</con:value>
        </con:Height>
      </con:height>
    </bldg:Building>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_construction =
            deserialize_abstract_construction(xml_document, &spans).expect("should work");
        let creation_date = abstract_construction
            .creation_date()
            .expect("should have creation date");
        assert_eq!(creation_date.year(), 2025);
        assert_eq!(creation_date.month(), 3);
        assert_eq!(creation_date.day(), 2);
        assert_eq!(creation_date.hour(), 15);
        assert_eq!(creation_date.minute(), 16);
        assert_eq!(creation_date.second(), 17);
    }

    #[test]
    fn test_deserialize_abstract_construction_with_occupancy() {
        let xml_document = b"\
<bldg:Building gml:id=\"DEBY_LOD2_4959457\">
    <con:occupancy>
        <Occupancy>
            <numberOfOccupants>123</numberOfOccupants>
            <interval>myInterval</interval>
            <occupantType>myOccupantType</occupantType>
        </Occupancy>
    </con:occupancy>
</bldg:Building>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_construction =
            deserialize_abstract_construction(xml_document, &spans).expect("should work");

        assert_eq!(abstract_construction.occupancies().len(), 1);

        let occupancy = &abstract_construction.occupancies()[0];
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
    fn test_serialize_abstract_construction_with_occupancy() {
        let xml_document = b"\
<bldg:Building gml:id=\"DEBY_LOD2_4959457\">
    <con:occupancy>
        <Occupancy>
            <numberOfOccupants>123</numberOfOccupants>
            <interval>myInterval</interval>
            <occupantType>myOccupantType</occupantType>
        </Occupancy>
    </con:occupancy>
</bldg:Building>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_construction =
            deserialize_abstract_construction(xml_document, &spans).expect("should work");

        let xml_node_parts =
            serialize_abstract_construction(&abstract_construction, Formatting::Compact)
                .expect("should serialize");
        let xml = egml::io::util::XmlNode::new("bldg:Building", xml_node_parts)
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        assert!(xml.contains("con:occupancy"));
        assert!(xml.contains("Occupancy"));
        assert!(xml.contains("numberOfOccupants"));
        assert!(xml.contains("123"));
        assert!(xml.contains("myInterval"));
        assert!(xml.contains("myOccupantType"));
    }

    #[test]
    fn test_round_trip_abstract_construction_with_occupancy() {
        let xml_document = b"\
<bldg:Building gml:id=\"DEBY_LOD2_4959457\">
    <con:occupancy>
        <Occupancy>
            <numberOfOccupants>123</numberOfOccupants>
            <interval>myInterval</interval>
            <occupantType>myOccupantType</occupantType>
        </Occupancy>
    </con:occupancy>
</bldg:Building>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_construction =
            deserialize_abstract_construction(xml_document, &spans).expect("should work");

        let xml_node_parts =
            serialize_abstract_construction(&abstract_construction, Formatting::Compact)
                .expect("should serialize");
        let xml = egml::io::util::XmlNode::new("bldg:Building", xml_node_parts)
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        let round_tripped_spans = extract_xml_element_spans(xml.as_bytes()).expect("should work");
        let round_tripped = deserialize_abstract_construction(xml.as_bytes(), &round_tripped_spans)
            .expect("should work");

        assert_eq!(
            abstract_construction.occupancies(),
            round_tripped.occupancies()
        );
    }
}
