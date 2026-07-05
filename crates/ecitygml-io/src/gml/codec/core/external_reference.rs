use crate::Error;
use ecitygml_core::model::core::ExternalReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlExternalReference {
    #[serde(rename = "targetResource")]
    pub target_resource: String,

    #[serde(rename = "informationSystem", skip_serializing_if = "Option::is_none")]
    pub information_system: Option<String>,

    #[serde(rename = "relationType", skip_serializing_if = "Option::is_none")]
    pub relation_type: Option<String>,
}

impl TryFrom<GmlExternalReference> for ExternalReference {
    type Error = Error;

    fn try_from(item: GmlExternalReference) -> Result<Self, Self::Error> {
        Ok(ExternalReference {
            target_resource: item.target_resource,
            information_system: item.information_system,
            relation_type: item.relation_type,
        })
    }
}

impl From<&ExternalReference> for GmlExternalReference {
    fn from(item: &ExternalReference) -> Self {
        GmlExternalReference {
            target_resource: item.target_resource.clone(),
            information_system: item.information_system.clone(),
            relation_type: item.relation_type.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gml::codec::core::external_reference::GmlExternalReference;
    use ecitygml_core::model::core::ExternalReference;
    use quick_xml::de;

    #[test]
    fn test_deserialize_external_reference_basic() {
        let xml_document = "<ExternalReference>
          <targetResource>DEHHALKA70002P5E</targetResource>
          <informationSystem>http://repository.gdi-de.org/schemas/adv/citygml/fdv/art.htm#_9100</informationSystem>
        </ExternalReference>";
        let gml: GmlExternalReference =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let external_reference: ExternalReference = gml.try_into().expect("should work");

        assert_eq!(external_reference.target_resource, "DEHHALKA70002P5E");
        assert!(external_reference.information_system.is_some());
        assert_eq!(
            external_reference.information_system.unwrap(),
            "http://repository.gdi-de.org/schemas/adv/citygml/fdv/art.htm#_9100"
        );
        assert!(external_reference.relation_type.is_none());
    }
}
