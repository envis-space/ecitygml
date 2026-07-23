use crate::gml::codec::core::qualified_volume::GmlQualifiedVolume;
use ecitygml_core::model::core::QualifiedVolume;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlVolumeProperty {
    #[serde(rename = "QualifiedVolume")]
    pub qualified_volume: GmlQualifiedVolume,
}

impl From<GmlVolumeProperty> for QualifiedVolume {
    fn from(item: GmlVolumeProperty) -> Self {
        Self::from(item.qualified_volume)
    }
}

impl From<&QualifiedVolume> for GmlVolumeProperty {
    fn from(item: &QualifiedVolume) -> Self {
        Self {
            qualified_volume: GmlQualifiedVolume::from(item),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::values::QualifiedVolumeTypeValue;
    use egml::model::basic_types::Code;
    use egml::model::measures::Volume;
    use quick_xml::{de, se};

    #[test]
    fn test_deserialize_volume_property() {
        let xml_document = b"<volume>
    <QualifiedVolume>
        <volume uom=\"m3\">5.0</volume>
        <typeOfVolume>RoadVolume</typeOfVolume>
    </QualifiedVolume>
</volume>";

        let gml_volume_property: GmlVolumeProperty =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let qualified_volume = QualifiedVolume::from(gml_volume_property);

        assert_eq!(qualified_volume.volume().value(), 5.0);
        assert_eq!(qualified_volume.volume().uom(), "m3");
        assert_eq!(
            qualified_volume.type_of_volume().code().value(),
            "RoadVolume"
        );
    }

    #[test]
    fn test_serialize_volume_property() {
        let qualified_volume = QualifiedVolume::new(
            Volume::new(5.0, "m3"),
            QualifiedVolumeTypeValue::from(Code::new("RoadVolume")),
        );

        let xml = se::to_string_with_root("volume", &GmlVolumeProperty::from(&qualified_volume))
            .expect("should serialize");

        assert!(xml.contains("QualifiedVolume"));
        assert!(xml.contains("uom=\"m3\""));
        assert!(xml.contains("typeOfVolume"));
        assert!(xml.contains("RoadVolume"));
    }

    #[test]
    fn test_round_trip_volume_property() {
        let xml_document = b"<volume>
    <QualifiedVolume>
        <volume uom=\"m3\">5.0</volume>
        <typeOfVolume>RoadVolume</typeOfVolume>
    </QualifiedVolume>
</volume>";

        let gml_volume_property: GmlVolumeProperty =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let qualified_volume = QualifiedVolume::from(gml_volume_property);

        let xml = se::to_string_with_root("volume", &GmlVolumeProperty::from(&qualified_volume))
            .expect("should serialize");

        let round_tripped_gml: GmlVolumeProperty = de::from_str(&xml).expect("should work");
        let round_tripped = QualifiedVolume::from(round_tripped_gml);

        assert_eq!(qualified_volume, round_tripped);
    }
}
