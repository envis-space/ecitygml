use crate::Error;
use crate::gml::codec::building::abstract_building_subdivision::{
    deserialize_abstract_building_subdivision, serialize_abstract_building_subdivision,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, extract_xml_element_spans};
use crate::gml::write::Formatting;
use ecitygml_core::model::building::{AsAbstractBuildingSubdivision, Storey};

pub fn deserialize_storey(xml_document: &[u8]) -> Result<Storey, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_building_subdivision =
        deserialize_abstract_building_subdivision(xml_document, &spans)?;
    let storey = Storey::from_abstract_building_subdivision(abstract_building_subdivision);

    Ok(storey)
}

pub fn serialize_storey(storey: &Storey, formatting: Formatting) -> Result<XmlNode, Error> {
    let xml_node_parts = serialize_abstract_building_subdivision(
        storey.abstract_building_subdivision(),
        formatting,
    )?;
    Ok(XmlNode::new(XmlElement::Storey, xml_node_parts))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::AsAbstractFeature;
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_storey_with_building_constructive_element() {
        let xml_document = b"<bldg:Storey gml:id=\"test-id\">>
          <gml:name>Ground Floor</gml:name>
          <bldg:class>Entrance Floor</bldg:class>
          <bldg:buildingConstructiveElement>
            <bldg:BuildingConstructiveElement
              gml:id=\"_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj\">
              <gml:name>01 - Ground Floor Ground plate</gml:name>
              <core:lod2Solid>
                <gml:Solid>
                  <gml:exterior>
                    <gml:Shell>
                      <gml:surfaceMember>
                        <gml:Polygon
                          gml:id=\"_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj_PG.xJxsIZemRugiHB0gL5h6\">
                          <gml:exterior>
                            <gml:LinearRing>
                              <gml:posList srsDimension=\"3\">458912.0 5438785.0 113.5 458900.0
                                5438785.0 113.5 458900.0 5438795.0 113.5 458912.0 5438795.0 113.5
                                458912.0 5438785.0 113.5</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                      <gml:surfaceMember>
                        <gml:Polygon
                          gml:id=\"_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj_PG.QOcD5lDTtfLZxmwl1Zwu\">
                          <gml:exterior>
                            <gml:LinearRing>
                              <gml:posList srsDimension=\"3\">458900.0 5438785.0 113.7 458912.0
                                5438785.0 113.7 458912.0 5438795.0 113.7 458900.0 5438795.0 113.7
                                458900.0 5438785.0 113.7</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                      <gml:surfaceMember>
                        <gml:Polygon
                          gml:id=\"_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj_PG.e9AMeV0pEURTngbpCIid\">
                          <gml:exterior>
                            <gml:LinearRing
                              gml:id=\"_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj_LR.1wtpWcM1ybr7En9jkc48\">
                              <gml:posList srsDimension=\"3\">458900.0 5438795.0 113.5 458900.0
                                5438795.0 113.7 458912.0 5438795.0 113.7 458912.0 5438795.0 113.5
                                458900.0 5438795.0 113.5</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                      <gml:surfaceMember>
                        <gml:Polygon
                          gml:id=\"_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj_PG.hNmG4YmoXQMBm8b42JJ4\">
                          <gml:exterior>
                            <gml:LinearRing
                              gml:id=\"_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj_LR.8mE1jXWCpsO6DUwaB6i7\">
                              <gml:posList srsDimension=\"3\">458912.0 5438795.0 113.5 458912.0
                                5438795.0 113.7 458912.0 5438785.0 113.7 458912.0 5438785.0 113.5
                                458912.0 5438795.0 113.5</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                      <gml:surfaceMember>
                        <gml:Polygon
                          gml:id=\"_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj_PG.vgpO7agF1kI7rJRF6jCg\">
                          <gml:exterior>
                            <gml:LinearRing
                              gml:id=\"_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj_LR.N0N0IsKBdvpy39boJBww\">
                              <gml:posList srsDimension=\"3\">458912.0 5438785.0 113.5 458912.0
                                5438785.0 113.7 458900.0 5438785.0 113.7 458900.0 5438785.0 113.5
                                458912.0 5438785.0 113.5</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                      <gml:surfaceMember>
                        <gml:Polygon
                          gml:id=\"_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj_PG.MgxBTndXeCqtGtBjgXrT\">
                          <gml:exterior>
                            <gml:LinearRing
                              gml:id=\"_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj_LR.fGwcpAcL6rIhCv6OIXnh\">
                              <gml:posList srsDimension=\"3\">458900.0 5438785.0 113.5 458900.0
                                5438785.0 113.7 458900.0 5438795.0 113.7 458900.0 5438795.0 113.5
                                458900.0 5438785.0 113.5</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Polygon>
                      </gml:surfaceMember>
                    </gml:Shell>
                  </gml:exterior>
                </gml:Solid>
              </core:lod2Solid>
              <con:isStructuralElement>true</con:isStructuralElement>
              <bldg:class>Slab</bldg:class>
            </bldg:BuildingConstructiveElement>
          </bldg:buildingConstructiveElement>
          <bldg:buildingConstructiveElement xlink:href=\"#UUID_7996a7e2-b625-437b-b5c3-f3e94b3ee7a8\"/>
        </bldg:Storey>";

        let storey = deserialize_storey(xml_document).expect("should work");

        assert_eq!(storey.id(), &Id::try_from("test-id").expect("should work"));
        /*assert_eq!(
            storey.name().first().expect("should work"),
            "AC14-FZK-Haus"
        );
        assert_eq!(storey.wall_surface.len(), 2);
        assert_eq!(storey.generic_attributes().len(), 3);

        let first_wall_surface = storey.wall_surface.first().unwrap();
        assert_eq!(
            first_wall_surface.id(),
            &Id::try_from("GML_5856d7ad-5e34-498a-817b-9544bfbb1475").expect("should work")
        );*/
    }
}
