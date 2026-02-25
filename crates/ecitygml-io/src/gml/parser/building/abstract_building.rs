use crate::Error;
use crate::gml::parser::construction::parse_abstract_construction;
use ecitygml_core::model::building::{AbstractBuilding, AsAbstractBuildingMut};
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn parse_abstract_building(xml_document: &[u8]) -> Result<AbstractBuilding, Error> {
    let abstract_construction = parse_abstract_construction(xml_document)?;
    let parsed_result: GmlAbstractBuilding = de::from_reader(xml_document)?;

    let mut abstract_building = AbstractBuilding::new(abstract_construction);

    abstract_building.set_function(
        parsed_result
            .function
            .into_iter()
            .map(|x| x.into())
            .collect(),
    );
    abstract_building.set_usage(parsed_result.usage.into_iter().map(|x| x.into()).collect());
    abstract_building.set_roof_type(parsed_result.roof_type.map(|x| x.into()));
    abstract_building.set_storeys_above_ground(parsed_result.storeys_above_ground);
    abstract_building.set_storeys_below_ground(parsed_result.storeys_below_ground);

    Ok(abstract_building)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractBuilding {
    #[serde(rename = "function", default)]
    pub function: Vec<GmlCode>,

    #[serde(rename = "usage", default)]
    pub usage: Vec<GmlCode>,

    #[serde(rename = "roofType", default)]
    pub roof_type: Option<GmlCode>,

    #[serde(rename = "storeysAboveGround", default)]
    pub storeys_above_ground: Option<i64>,

    #[serde(rename = "storeysBelowGround", default)]
    pub storeys_below_ground: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::building::AsAbstractBuilding;
    use ecitygml_core::model::construction::AsAbstractConstruction;

    #[test]
    fn test_parse_basic_abstract_building() {
        let xml_document = b"
    <bldg:Building gml:id=\"DEBY_LOD2_4959457\">
      <creationDate>2025-01-02T00:00:00+01:00</creationDate>
      <externalReference>
        <ExternalReference>
          <targetResource>DEBYvAAAAABTMIyA</targetResource>
          <informationSystem>http://repository.gdi-de.org/schemas/adv/citygml/fdv/art.htm#_9100</informationSystem>
        </ExternalReference>
      </externalReference>
      <bldg:function>31001_3020</bldg:function>
      <bldg:roofType>1000</bldg:roofType>
      <bldg:storeysAboveGround>4</bldg:storeysAboveGround>
      <bldg:address>
        <Address>
          <xalAddress>
            <xAL:Address>
              <xAL:Country>
                <xAL:NameElement xAL:NameType=\"Name\">Germany</xAL:NameElement>
              </xAL:Country>
              <xAL:Locality xAL:Type=\"Town\">
                <xAL:NameElement xAL:NameType=\"Name\">Muenchen</xAL:NameElement>
              </xAL:Locality>
              <xAL:Thoroughfare xAL:Type=\"Street\">
                <xAL:NameElement>Arcisstrasse 21</xAL:NameElement>
              </xAL:Thoroughfare>
            </xAL:Address>
          </xalAddress>
        </Address>
      </bldg:address>
    </bldg:Building>";

        let abstract_building = parse_abstract_building(xml_document).expect("should work");
        assert!(abstract_building.creation_date().is_some());
        assert_eq!(
            abstract_building.function().first().unwrap().value,
            "31001_3020"
        );
        assert_eq!(abstract_building.usage().is_empty(), true);
        assert_eq!(
            abstract_building
                .roof_type()
                .as_ref()
                .expect("should be set")
                .value,
            "1000"
        );
        assert_eq!(abstract_building.storeys_above_ground().unwrap(), 4);
        assert!(abstract_building.storeys_below_ground().is_none());
    }
}
