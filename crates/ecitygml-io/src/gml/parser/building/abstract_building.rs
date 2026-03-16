use crate::Error;
use crate::gml::parser::city_object_reader::read_city_objects;
use crate::gml::parser::construction::deserialize_abstract_construction;
use ecitygml_core::model::building::{AbstractBuilding, AsAbstractBuildingMut};
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::core::CityObjectKind;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub fn deserialize_abstract_building(xml_document: &[u8]) -> Result<AbstractBuilding, Error> {
    let abstract_construction = deserialize_abstract_construction(xml_document)?;
    let parsed_result: GmlAbstractBuilding = de::from_reader(xml_document)?;

    let mut abstract_building = AbstractBuilding::new(abstract_construction);

    abstract_building.set_class(parsed_result.class.map(|x| x.into()));
    abstract_building.set_functions(
        parsed_result
            .functions
            .into_iter()
            .map(|x| x.into())
            .collect(),
    );
    abstract_building.set_usages(parsed_result.usages.into_iter().map(|x| x.into()).collect());
    abstract_building.set_roof_type(parsed_result.roof_type.map(|x| x.into()));
    abstract_building.set_storeys_above_ground(parsed_result.storeys_above_ground);
    abstract_building.set_storeys_below_ground(parsed_result.storeys_below_ground);

    let parsed_city_objects = read_city_objects(
        xml_document,
        HashSet::from([
            CityObjectClass::BuildingConstructiveElement,
            CityObjectClass::BuildingInstallation,
            CityObjectClass::BuildingRoom,
            CityObjectClass::Storey,
        ]),
    )?;
    for city_object in parsed_city_objects {
        match city_object {
            CityObjectKind::BuildingConstructiveElement(x) => {
                abstract_building.building_constructive_elements.push(x);
            }
            CityObjectKind::BuildingInstallation(x) => {
                abstract_building.building_installations.push(x);
            }
            CityObjectKind::BuildingRoom(x) => {
                abstract_building.building_rooms.push(x);
            }
            CityObjectKind::Storey(x) => {
                abstract_building.building_subdivisions.push(x.into());
            }
            _ => {
                return Err(Error::UnknownElementNode(format!("{:?}", city_object)));
            }
        }
    }

    Ok(abstract_building)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractBuilding {
    #[serde(rename = "class", default)]
    pub class: Option<GmlCode>,

    #[serde(rename = "function", default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename = "usage", default)]
    pub usages: Vec<GmlCode>,

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
    use ecitygml_core::model::core::AsAbstractFeatureWithLifespan;

    #[test]
    fn test_deserialize_basic_abstract_building() {
        let xml_document = b"
    <bldg:Building gml:id=\"DEBY_LOD2_4959457\">
      <creationDate>2025-01-02T00:00:00+01:00</creationDate>
      <externalReference>
        <ExternalReference>
          <targetResource>DEBYvAAAAABTMIyA</targetResource>
          <informationSystem>http://repository.gdi-de.org/schemas/adv/citygml/fdv/art.htm#_9100</informationSystem>
        </ExternalReference>
      </externalReference>
      <bldg:class>MyBuildingClass</bldg:class>
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

        let abstract_building = deserialize_abstract_building(xml_document).expect("should work");
        assert!(abstract_building.date_of_construction().is_none());
        assert!(abstract_building.creation_date().is_some());
        assert_eq!(
            abstract_building.class().as_ref().unwrap().value,
            "MyBuildingClass"
        );
        assert_eq!(
            abstract_building.functions().first().unwrap().value,
            "31001_3020"
        );
        assert_eq!(abstract_building.usages().is_empty(), true);
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
