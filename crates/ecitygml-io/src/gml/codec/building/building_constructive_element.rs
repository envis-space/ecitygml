use crate::Error;
use crate::gml::codec::construction::{
    deserialize_abstract_constructive_element, serialize_abstract_constructive_element,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::building::BuildingConstructiveElement;
use ecitygml_core::model::building::values::{
    BuildingConstructiveElementClassValue, BuildingConstructiveElementFunctionValue,
    BuildingConstructiveElementUsageValue,
};
use ecitygml_core::model::construction::AsAbstractConstructiveElement;
use egml::io::codec::basic::GmlCode;
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::basic_types::Code;
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
        BuildingConstructiveElement::from_abstract_constructive_element(
            abstract_constructive_element,
        );

    building_constructive_element.set_class_opt(
        parsed
            .class
            .map(Code::from)
            .map(BuildingConstructiveElementClassValue::from),
    );
    building_constructive_element.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(BuildingConstructiveElementFunctionValue::from)
            .collect(),
    );
    building_constructive_element.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(BuildingConstructiveElementUsageValue::from)
            .collect(),
    );

    Ok(building_constructive_element)
}

pub fn serialize_building_constructive_element(
    building_constructive_element: &BuildingConstructiveElement,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_constructive_element(
        building_constructive_element.abstract_constructive_element(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlBuildingConstructiveElement::from(building_constructive_element),
        formatting,
    )? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(
        CityGmlElement::BuildingConstructiveElement.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingConstructiveElement {
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
            class: item
                .class()
                .map(BuildingConstructiveElementClassValue::code)
                .map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(BuildingConstructiveElementFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(BuildingConstructiveElementUsageValue::code)
                .map(Into::into)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::construction::AsAbstractConstructiveElementMut;
    use ecitygml_core::model::core::AsAbstractFeature;
    use egml::model::base::Id;
    use egml::model::basic_types::Code;
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
            building_constructive_element.feature_id(),
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
        let mut building_constructive_element = BuildingConstructiveElement::new(id);
        building_constructive_element.set_is_structural_element(Some(true));
        building_constructive_element.set_class(Code::new("Slab").into());

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
