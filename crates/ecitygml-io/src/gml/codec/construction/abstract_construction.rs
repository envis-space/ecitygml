use crate::Error;
use crate::gml::codec::core::deserialize_abstract_occupied_space;
use crate::gml::util::XmlElementSpans;
use chrono::NaiveDate;
use ecitygml_core::model::construction::{AbstractConstruction, AsAbstractConstructionMut};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_construction(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractConstruction, Error> {
    let (abstract_occupied_space_result, parsed_result) = rayon::join(
        || deserialize_abstract_occupied_space(xml_document, spans),
        || de::from_reader::<_, GmlAbstractConstruction>(xml_document).map_err(Error::from),
    );
    let abstract_occupied_space = abstract_occupied_space_result?;
    let parsed = parsed_result?;
    let mut abstract_construction = AbstractConstruction::new(abstract_occupied_space);

    abstract_construction.set_date_of_construction(parsed.date_of_construction);
    abstract_construction.set_date_of_demolition(parsed.date_of_demolition);

    Ok(abstract_construction)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractConstruction {
    #[serde(rename = "dateOfConstruction", skip_serializing_if = "Option::is_none")]
    pub date_of_construction: Option<NaiveDate>,

    #[serde(rename = "dateOfDemolition", skip_serializing_if = "Option::is_none")]
    pub date_of_demolition: Option<NaiveDate>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::codec::building::deserialize_building;
    use crate::gml::codec::core::deserialize_abstract_thematic_surface;
    use crate::gml::util::extract_xml_element_spans;
    use chrono::{Datelike, Timelike};
    use ecitygml_core::model::construction::AsAbstractConstruction;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractFeatureWithLifespan,
        AsAbstractThematicSurface,
    };
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
}
