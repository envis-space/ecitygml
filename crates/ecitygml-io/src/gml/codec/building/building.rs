use crate::Error;
use crate::gml::codec::building::building_part_property::{
    deserialize_building_part_property, serialize_building_part_property,
};
use crate::gml::codec::building::{deserialize_abstract_building, serialize_abstract_building};
use crate::gml::codec::transportation::GmlRoad;
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::building::{AsAbstractBuilding, Building};
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, collect_children, extract_xml_element_spans,
};
use serde::{Deserialize, Serialize};

pub fn deserialize_building(xml_document: &[u8]) -> Result<Building, Error> {
    let spans = extract_xml_element_spans(xml_document)?;

    let mut abstract_building_result = None;
    let mut parsed_result = None;
    let mut building_parts_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_building_result = Some(deserialize_abstract_building(xml_document, &spans))
        });
        s.spawn(|_| {
            parsed_result =
                Some(quick_xml::de::from_reader::<_, GmlRoad>(xml_document).map_err(Error::from));
        });
        s.spawn(|_| {
            building_parts_result = Some(collect_children(
                xml_document,
                &spans,
                CityGmlElement::BuildingPartProperty.into(),
                deserialize_building_part_property,
            ));
        });
    });

    let abstract_building =
        abstract_building_result.expect("rayon::scope guarantees all spawns complete")?;
    let _parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let building_parts =
        building_parts_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut building = Building::from_abstract_building(abstract_building);
    building.set_building_parts(building_parts);

    Ok(building)
}

pub fn serialize_building(building: &Building, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_building(building.abstract_building(), formatting)?;

    for prop in building.building_parts() {
        let node = serialize_building_part_property(prop, formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Child(node));
    }

    Ok(XmlNode::new(
        CityGmlElement::Building.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuilding {}

impl From<&Building> for GmlBuilding {
    fn from(_item: &Building) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::building::AsAbstractBuildingMut;
    use ecitygml_core::model::construction::{AbstractConstructionSurfaceKind, WallSurface};
    use ecitygml_core::model::core::{
        AbstractSpaceBoundaryKind, AbstractThematicSurfaceKind, AsAbstractCityObject,
        AsAbstractFeature, AsAbstractFeatureMut, AsAbstractSpace,
    };
    use egml::model::base::{AsAbstractGml, AsAbstractGmlMut, Id};
    use egml::model::basic_types::Code;

    #[test]
    fn test_deserialize_basic_building() {
        let xml_document = b"
    <bldg:Building gml:id=\"UUID_d281adfc-4901-0f52-540b-4cc1a9325f82\">
      <gml:name>AC14-FZK-Haus</gml:name>
      <creationDate>2017-01-23T00:00:00+01:00</creationDate>
      <relativeToTerrain>entirelyAboveTerrain</relativeToTerrain>
      <genericAttribute>
        <gen:MeasureAttribute>
          <gen:name>GrossPlannedArea</gen:name>
          <gen:value uom=\"m2\">120.0</gen:value>
        </gen:MeasureAttribute>
      </genericAttribute>
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>ConstructionMethod</gen:name>
          <gen:value>New Building</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>IsLandmarked</gen:name>
          <gen:value>NO</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <boundary>
        <con:WallSurface gml:id=\"GML_5856d7ad-5e34-498a-817b-9544bfbb1475\">
          <gml:name>Outer Wall 1 (West)</gml:name>
          <lod2MultiSurface>
            <gml:MultiSurface>
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"PolyID7350_878_759628_120742\">
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"PolyID7350_878_759628_120742_0\">
                      <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                      <gml:pos>457842.0 5439093.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439093.0 111.8</gml:pos>
                      <gml:pos>457842.0 5439083.0 111.8</gml:pos>
                      <gml:pos>457842.0 5439083.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"GML_d38cf762-c29d-4491-88c9-bdc89e141978\">
          <gml:name>Outer Wall 2 (South)</gml:name>
          <lod2MultiSurface>
            <gml:MultiSurface>
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"PolyID7351_1722_416019_316876\">
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"PolyID7351_1722_416019_316876_0\">
                      <gml:pos>457854.0 5439083.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439083.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439083.0 111.8</gml:pos>
                      <gml:pos>457854.0 5439083.0 111.8</gml:pos>
                      <gml:pos>457854.0 5439083.0 115.430940107676</gml:pos>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
    </bldg:Building>";

        let building = deserialize_building(xml_document).expect("should work");

        assert_eq!(
            building.feature_id(),
            &Id::try_from("UUID_d281adfc-4901-0f52-540b-4cc1a9325f82").expect("should work")
        );
        assert_eq!(
            building.names().first().expect("should work"),
            &Code::new("AC14-FZK-Haus")
        );
        let wall_surfaces: Vec<&WallSurface> = building
            .boundaries()
            .iter()
            .flat_map(|x| x.object())
            .filter_map(|x| match x {
                AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(
                    AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(
                        AbstractConstructionSurfaceKind::WallSurface(w),
                    ),
                ) => Some(w),
                _ => None,
            })
            .collect();
        assert_eq!(wall_surfaces.len(), 2);
        assert_eq!(building.generic_attributes().len(), 3);

        let first_wall_surface = wall_surfaces.first().unwrap();
        assert_eq!(
            first_wall_surface.feature_id(),
            &Id::try_from("GML_5856d7ad-5e34-498a-817b-9544bfbb1475").expect("should work")
        );
    }

    #[test]
    fn test_deserialize_basic_building_2() {
        let xml_document = b"
    <bldg:Building gml:id=\"DEBY_LOD2_4959457\">
      <creationDate>2025-01-02T00:00:00+01:00</creationDate>
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

        let building = deserialize_building(xml_document).expect("should work");
    }

    #[test]
    fn test_serialize_basic_building() {
        let id = Id::try_from("abc").expect("should work");
        let mut building = Building::new(id);

        building.set_names(vec!["good".into(), "morning".into()]);
        building.set_roof_type(Code::new("1000").into());
        building.set_storeys_above_ground(4);

        let gml = serialize_building(&building, Formatting::Indent { char: ' ', size: 2 })
            .expect("should work");
        // println!("{}", String::from_utf8_lossy(&gml));
        println!("");
    }
}
