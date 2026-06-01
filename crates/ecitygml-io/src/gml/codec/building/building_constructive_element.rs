use crate::Error;
use crate::gml::codec::construction::{
    GmlAbstractConstructiveElement, deserialize_abstract_constructive_element,
};
use crate::gml::util::extract_xml_element_spans;
use ecitygml_core::model::building::BuildingConstructiveElement;
use ecitygml_core::model::construction::AsAbstractConstructiveElement;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_constructive_element(
    xml_document: &[u8],
) -> Result<BuildingConstructiveElement, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_constructive_element_result, parsed_result) = rayon::join(
        || deserialize_abstract_constructive_element(xml_document, &spans),
        || de::from_reader::<_, GmlBuildingConstructiveElement>(xml_document).map_err(Error::from),
    );
    let abstract_constructive_element = abstract_constructive_element_result?;
    let parsed = parsed_result?;
    let mut building_constructive_element =
        BuildingConstructiveElement::new(abstract_constructive_element);

    building_constructive_element.set_class(parsed.class.map(|x| x.into()));
    building_constructive_element
        .set_functions(parsed.functions.into_iter().map(|x| x.into()).collect());
    building_constructive_element.set_usages(parsed.usages.into_iter().map(|x| x.into()).collect());

    Ok(building_constructive_element)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingConstructiveElement {
    #[serde(flatten, skip_deserializing, default)]
    pub abstract_constructive_element: Option<GmlAbstractConstructiveElement>,

    #[serde(
        rename(serialize = "bldg:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "bldg:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "bldg:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,
}

impl From<&BuildingConstructiveElement> for GmlBuildingConstructiveElement {
    fn from(item: &BuildingConstructiveElement) -> Self {
        Self {
            abstract_constructive_element: Some(item.abstract_constructive_element().into()),
            class: item.class().as_ref().map(Into::into),
            functions: Vec::new(),
            usages: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::construction::{
        AbstractConstructiveElement, AsAbstractConstructiveElementMut,
    };
    use ecitygml_core::model::core::{
        AbstractCityObject, AbstractFeature, AbstractFeatureWithLifespan, AbstractOccupiedSpace,
        AbstractPhysicalSpace, AbstractSpace, AsAbstractFeature,
    };
    use egml::model::base::Id;
    use egml::model::basic::Code;
    use quick_xml::se;

    #[test]
    fn test_deserialize_building_constructive_element() {
        let xml_document = b"
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
        </bldg:BuildingConstructiveElement>";

        let building_constructive_element =
            deserialize_building_constructive_element(xml_document).expect("should work");

        assert_eq!(
            building_constructive_element.id(),
            &Id::try_from("_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj").expect("should work")
        );
        /*assert_eq!(
            building_constructive_element.name().first().expect("should work"),
            "AC14-FZK-Haus"
        );
        assert_eq!(building_constructive_element.wall_surface.len(), 2);
        assert_eq!(building_constructive_element.generic_attributes().len(), 3);

        let first_wall_surface = building_constructive_element.wall_surface.first().unwrap();
        assert_eq!(
            first_wall_surface.id(),
            &Id::try_from("GML_5856d7ad-5e34-498a-817b-9544bfbb1475").expect("should work")
        );*/
    }

    #[test]
    fn test_serialize_building_constructive_element() {
        let id = Id::try_from("test-id").expect("should work");
        let abstract_feature = AbstractFeature::new(id);
        let abstract_feature_with_lifespan = AbstractFeatureWithLifespan::new(abstract_feature);
        let abstract_city_object = AbstractCityObject::new(abstract_feature_with_lifespan);
        let abstract_space = AbstractSpace::new(abstract_city_object);
        let abstract_physical_space = AbstractPhysicalSpace::new(abstract_space);
        let abstract_occupied_space = AbstractOccupiedSpace::new(abstract_physical_space);
        let mut abstract_constructive_element =
            AbstractConstructiveElement::new(abstract_occupied_space);
        abstract_constructive_element.set_is_structural_element(Some(true));
        let mut building_constructive_element =
            BuildingConstructiveElement::new(abstract_constructive_element);
        building_constructive_element.set_class(Some(Code::new("Slab")));

        let gml_building_constructive_element =
            GmlBuildingConstructiveElement::from(&building_constructive_element);
        let xml_document = se::to_string_with_root(
            "bldg:BuildingConstructiveElement",
            &gml_building_constructive_element,
        )
        .expect("should work");

        print!("ac");

        /*let building_constructive_element =
            deserialize_building_constructive_element(xml_document).expect("should work");

        assert_eq!(
            building_constructive_element.id(),
            &Id::try_from("_FZK-Haus-Storey-Construction-tex-Windows_BD.CvWEAVLAPO2cqWPW8xrz_BP.Um20q9TChr2VYbrcN9zj").expect("should work")
        );*/
        /*assert_eq!(
            building_constructive_element.name().first().expect("should work"),
            "AC14-FZK-Haus"
        );
        assert_eq!(building_constructive_element.wall_surface.len(), 2);
        assert_eq!(building_constructive_element.generic_attributes().len(), 3);

        let first_wall_surface = building_constructive_element.wall_surface.first().unwrap();
        assert_eq!(
            first_wall_surface.id(),
            &Id::try_from("GML_5856d7ad-5e34-498a-817b-9544bfbb1475").expect("should work")
        );*/
    }
}
