use crate::Error;
use crate::gml::codec::building::building_constructive_element_property::deserialize_building_constructive_element_property;
use crate::gml::codec::building::building_installation_property::deserialize_building_installation_property;
use crate::gml::codec::building::building_room_property::deserialize_building_room_property;
use crate::gml::codec::building::building_subdivision_property::deserialize_building_subdivision_property;
use crate::gml::codec::construction::deserialize_abstract_construction;
use crate::gml::util::collect_children;
use crate::gml::util::{XmlElement, XmlElementSpans};
use ecitygml_core::model::building::{AbstractBuilding, AsAbstractBuildingMut};
use egml::io::GmlCode;
use quick_xml::de;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_building(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<AbstractBuilding, Error> {
    let mut abstract_construction_result = None;
    let mut parsed_result = None;
    let mut building_rooms_result = None;
    let mut building_installations_result = None;
    let mut building_constructive_elements_result = None;
    let mut building_subdivisions_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_construction_result =
                Some(deserialize_abstract_construction(xml_document, spans));
        });
        s.spawn(|_| {
            parsed_result =
                Some(de::from_reader::<_, GmlAbstractBuilding>(xml_document).map_err(Error::from));
        });
        s.spawn(|_| {
            building_rooms_result = Some(collect_children(
                xml_document,
                spans,
                XmlElement::BuildingRoomProperty,
                deserialize_building_room_property,
            ));
        });
        s.spawn(|_| {
            building_installations_result = Some(collect_children(
                xml_document,
                spans,
                XmlElement::BuildingInstallationProperty,
                deserialize_building_installation_property,
            ));
        });
        s.spawn(|_| {
            building_constructive_elements_result = Some(collect_children(
                xml_document,
                spans,
                XmlElement::BuildingConstructiveElementProperty,
                deserialize_building_constructive_element_property,
            ));
        });
        s.spawn(|_| {
            building_subdivisions_result = Some(collect_children(
                xml_document,
                spans,
                XmlElement::BuildingSubdivisionProperty,
                deserialize_building_subdivision_property,
            ));
        });
    });

    let abstract_construction =
        abstract_construction_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let building_rooms =
        building_rooms_result.expect("rayon::scope guarantees all spawns complete")?;
    let building_installations =
        building_installations_result.expect("rayon::scope guarantees all spawns complete")?;
    let building_constructive_elements = building_constructive_elements_result
        .expect("rayon::scope guarantees all spawns complete")?;
    let building_subdivisions =
        building_subdivisions_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut abstract_building = AbstractBuilding::new(abstract_construction);
    abstract_building.building_rooms = building_rooms;
    abstract_building.building_installations = building_installations;
    abstract_building.building_constructive_elements = building_constructive_elements;
    abstract_building.building_subdivisions = building_subdivisions;

    abstract_building.set_class(parsed.class.map(|x| x.into()));
    abstract_building.set_functions(parsed.functions.into_iter().map(|x| x.into()).collect());
    abstract_building.set_usages(parsed.usages.into_iter().map(|x| x.into()).collect());
    abstract_building.set_roof_type(parsed.roof_type.map(|x| x.into()));
    abstract_building.set_storeys_above_ground(parsed.storeys_above_ground);
    abstract_building.set_storeys_below_ground(parsed.storeys_below_ground);

    Ok(abstract_building)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractBuilding {
    #[serde(
        rename(serialize = "bldg:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "bldg:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "bldg:usage", deserialize = "usage"), default)]
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
    use crate::gml::util::extract_xml_element_spans;
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

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_building =
            deserialize_abstract_building(xml_document, &spans).expect("should work");
        assert!(abstract_building.date_of_construction().is_none());
        assert!(abstract_building.creation_date().is_some());
        assert_eq!(
            abstract_building.class().as_ref().unwrap().value(),
            "MyBuildingClass"
        );
        assert_eq!(
            abstract_building.functions().first().unwrap().value(),
            "31001_3020"
        );
        assert_eq!(abstract_building.usages().is_empty(), true);
        assert_eq!(
            abstract_building
                .roof_type()
                .as_ref()
                .expect("should be set")
                .value(),
            "1000"
        );
        assert_eq!(abstract_building.storeys_above_ground().unwrap(), 4);
        assert!(abstract_building.storeys_below_ground().is_none());
    }
}
