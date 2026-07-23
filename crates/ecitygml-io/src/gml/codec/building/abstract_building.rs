use crate::Error;
use crate::gml::codec::building::abstract_building_subdivision_property::{
    deserialize_abstract_building_subdivision_property,
    serialize_abstract_building_subdivision_property,
};
use crate::gml::codec::building::building_constructive_element_property::{
    deserialize_building_constructive_element_property,
    serialize_building_constructive_element_property,
};
use crate::gml::codec::building::building_installation_property::{
    deserialize_building_installation_property, serialize_building_installation_property,
};
use crate::gml::codec::building::building_room_property::{
    deserialize_building_room_property, serialize_building_room_property,
};
use crate::gml::codec::construction::{
    deserialize_abstract_construction, serialize_abstract_construction,
};
use crate::gml::util::{CityGmlElement, CombinedCityGmlElement};
use ecitygml_core::model::building::values::{
    BuildingClassValue, BuildingFunctionValue, BuildingUsageValue, RoofTypeValue,
};
use ecitygml_core::model::building::{AbstractBuilding, AsAbstractBuilding, AsAbstractBuildingMut};
use ecitygml_core::model::construction::AsAbstractConstruction;
use egml::io::codec::basic::GmlCode;
use egml::io::util::{
    Formatting, XmlElementSpans, XmlNodeContent, XmlNodeParts, collect_children, serialize_inner,
};
use egml::model::basic_types::Code;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_building(
    xml_document: &[u8],
    spans: &XmlElementSpans<CombinedCityGmlElement>,
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
                CityGmlElement::BuildingRoomProperty.into(),
                deserialize_building_room_property,
            ));
        });
        s.spawn(|_| {
            building_installations_result = Some(collect_children(
                xml_document,
                spans,
                CityGmlElement::BuildingInstallationProperty.into(),
                deserialize_building_installation_property,
            ));
        });
        s.spawn(|_| {
            building_constructive_elements_result = Some(collect_children(
                xml_document,
                spans,
                CityGmlElement::BuildingConstructiveElementProperty.into(),
                deserialize_building_constructive_element_property,
            ));
        });
        s.spawn(|_| {
            building_subdivisions_result = Some(collect_children(
                xml_document,
                spans,
                CityGmlElement::AbstractBuildingSubdivisionProperty.into(),
                deserialize_abstract_building_subdivision_property,
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

    let mut abstract_building = AbstractBuilding::from_abstract_construction(abstract_construction);
    abstract_building.set_building_constructive_elements(building_constructive_elements);
    abstract_building.set_building_installations(building_installations);
    abstract_building.set_building_rooms(building_rooms);
    abstract_building.set_building_subdivisions(building_subdivisions);

    abstract_building.set_class_opt(parsed.class.map(Code::from).map(BuildingClassValue::from));
    abstract_building.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(BuildingFunctionValue::from)
            .collect(),
    );
    abstract_building.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(BuildingUsageValue::from)
            .collect(),
    );
    abstract_building.set_roof_type_opt(parsed.roof_type.map(Code::from).map(RoofTypeValue::from));
    abstract_building.set_storeys_above_ground_opt(parsed.storeys_above_ground);
    abstract_building.set_storeys_below_ground_opt(parsed.storeys_below_ground);

    Ok(abstract_building)
}

pub fn serialize_abstract_building(
    abstract_building: &AbstractBuilding,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts =
        serialize_abstract_construction(abstract_building.abstract_construction(), formatting)?;

    if let Some(raw) = serialize_inner(GmlAbstractBuilding::from(abstract_building), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    for prop in abstract_building.building_rooms() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_building_room_property(
                prop, formatting,
            )?));
    }

    for prop in abstract_building.building_installations() {
        xml_node_parts.content.push(XmlNodeContent::Child(
            serialize_building_installation_property(prop, formatting)?,
        ));
    }

    for prop in abstract_building.building_constructive_elements() {
        xml_node_parts.content.push(XmlNodeContent::Child(
            serialize_building_constructive_element_property(prop, formatting)?,
        ));
    }

    for prop in abstract_building.building_subdivisions() {
        xml_node_parts.content.push(XmlNodeContent::Child(
            serialize_abstract_building_subdivision_property(prop, formatting)?,
        ));
    }

    Ok(xml_node_parts)
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

    #[serde(
        rename(serialize = "bldg:roofType", deserialize = "roofType"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub roof_type: Option<GmlCode>,

    #[serde(
        rename(
            serialize = "bldg:storeysAboveGround",
            deserialize = "storeysAboveGround"
        ),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storeys_above_ground: Option<i64>,

    #[serde(
        rename(
            serialize = "bldg:storeysBelowGround",
            deserialize = "storeysBelowGround"
        ),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storeys_below_ground: Option<i64>,
}

impl From<&AbstractBuilding> for GmlAbstractBuilding {
    fn from(item: &AbstractBuilding) -> Self {
        Self {
            class: item.class().map(BuildingClassValue::code).map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(BuildingFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(BuildingUsageValue::code)
                .map(Into::into)
                .collect(),
            roof_type: item.roof_type().map(RoofTypeValue::code).map(Into::into),
            storeys_above_ground: item.storeys_above_ground(),
            storeys_below_ground: item.storeys_below_ground(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::building::AsAbstractBuilding;
    use ecitygml_core::model::construction::AsAbstractConstruction;
    use ecitygml_core::model::core::AsAbstractFeatureWithLifespan;
    use egml::io::util::extract_xml_element_spans;

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
            abstract_building.class().unwrap().code().value(),
            "MyBuildingClass"
        );
        assert_eq!(
            abstract_building
                .functions()
                .first()
                .unwrap()
                .code()
                .value(),
            "31001_3020"
        );
        assert_eq!(abstract_building.usages().is_empty(), true);
        assert_eq!(
            abstract_building
                .roof_type()
                .expect("should be set")
                .code()
                .value(),
            "1000"
        );
        assert_eq!(abstract_building.storeys_above_ground().unwrap(), 4);
        assert!(abstract_building.storeys_below_ground().is_none());
    }
}
