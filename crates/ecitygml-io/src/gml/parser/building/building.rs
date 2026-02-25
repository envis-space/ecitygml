use crate::Error;
use crate::gml::parser::building::parse_abstract_building;
use crate::gml::parser::city_object_reader::read_city_objects;
use ecitygml_core::model::building::Building;
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::core::CityObjectKind;
use std::collections::HashSet;

pub fn parse_building(xml_document: &[u8]) -> Result<Building, Error> {
    let abstract_building = parse_abstract_building(xml_document)?;
    let mut building = Building::new(abstract_building);

    let parsed_city_objects = read_city_objects(
        xml_document,
        HashSet::from([
            CityObjectClass::BuildingConstructiveElement,
            CityObjectClass::GroundSurface,
            CityObjectClass::RoofSurface,
            CityObjectClass::WallSurface,
        ]),
    )?;

    for city_object in parsed_city_objects {
        match city_object {
            CityObjectKind::BuildingConstructiveElement(x) => {
                building.building_constructive_element.push(x);
            }
            CityObjectKind::GroundSurface(x) => {
                building.ground_surface.push(x);
            }
            CityObjectKind::RoofSurface(x) => {
                building.roof_surface.push(x);
            }
            CityObjectKind::WallSurface(x) => {
                building.wall_surface.push(x);
            }
            _ => {
                panic!("Unexpected city object kind: {:?}", city_object);
            }
        }
    }

    Ok(building)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::parser::core::parse_abstract_thematic_surface;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_parse_basic_building() {
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

        let building = parse_building(xml_document).expect("should work");

        assert_eq!(
            building.id(),
            &Id::try_from("UUID_d281adfc-4901-0f52-540b-4cc1a9325f82").expect("should work")
        );
        assert_eq!(
            building.name().first().expect("should work"),
            "AC14-FZK-Haus"
        );
        assert_eq!(building.wall_surface.len(), 2);
        assert_eq!(building.generic_attributes().len(), 3);

        let first_wall_surface = building.wall_surface.first().unwrap();
        assert_eq!(
            first_wall_surface.id(),
            &Id::try_from("GML_5856d7ad-5e34-498a-817b-9544bfbb1475").expect("should work")
        );
    }

    #[test]
    fn test_parse_basic_building_2() {
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

        let building = parse_building(xml_document).expect("should work");
    }
}
