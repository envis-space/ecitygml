use ecitygml_core::model::core::QualifiedVolume;
use ecitygml_core::model::core::values::QualifiedVolumeTypeValue;
use egml::io::codec::basic::GmlCode;
use egml::io::codec::measures::GmlVolume;
use egml::model::basic_types::Code;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlQualifiedVolume {
    #[serde(rename(serialize = "volume", deserialize = "volume"))]
    pub volume: GmlVolume,

    #[serde(rename = "typeOfVolume")]
    pub type_of_volume: GmlCode,
}

impl From<GmlQualifiedVolume> for QualifiedVolume {
    fn from(item: GmlQualifiedVolume) -> Self {
        Self::new(
            item.volume.into(),
            QualifiedVolumeTypeValue::from(Code::from(item.type_of_volume)),
        )
    }
}

impl From<&QualifiedVolume> for GmlQualifiedVolume {
    fn from(item: &QualifiedVolume) -> Self {
        Self {
            volume: item.volume().into(),
            type_of_volume: item.type_of_volume().code().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egml::model::basic_types::Code;
    use egml::model::measures::Volume;
    use quick_xml::{de, se};

    #[test]
    fn test_deserialize_qualified_volume() {
        let xml_document = b"<QualifiedVolume>
    <volume uom=\"m3\">5.0</volume>
    <typeOfVolume>RoadVolume</typeOfVolume>
</QualifiedVolume>";

        let gml_qualified_volume: GmlQualifiedVolume =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let qualified_volume = QualifiedVolume::from(gml_qualified_volume);

        assert_eq!(qualified_volume.volume().value(), 5.0);
        assert_eq!(qualified_volume.volume().uom(), "m3");
        assert_eq!(
            qualified_volume.type_of_volume().code().value(),
            "RoadVolume"
        );
    }

    #[test]
    fn test_serialize_qualified_volume() {
        let qualified_volume = QualifiedVolume::new(
            Volume::new(5.0, "m3"),
            QualifiedVolumeTypeValue::from(Code::new("RoadVolume")),
        );

        let xml = se::to_string_with_root(
            "QualifiedVolume",
            &GmlQualifiedVolume::from(&qualified_volume),
        )
        .expect("should serialize");

        assert!(xml.contains("QualifiedVolume"));
        assert!(xml.contains("volume"));
        assert!(xml.contains("uom=\"m3\""));
        assert!(xml.contains("5"));
        assert!(xml.contains("typeOfVolume"));
        assert!(xml.contains("RoadVolume"));
    }

    #[test]
    fn test_round_trip_qualified_volume() {
        let xml_document = b"<QualifiedVolume>
    <volume uom=\"m3\">5.0</volume>
    <typeOfVolume>RoadVolume</typeOfVolume>
</QualifiedVolume>";

        let gml_qualified_volume: GmlQualifiedVolume =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let qualified_volume = QualifiedVolume::from(gml_qualified_volume);

        let xml = se::to_string_with_root(
            "QualifiedVolume",
            &GmlQualifiedVolume::from(&qualified_volume),
        )
        .expect("should serialize");

        let round_tripped_gml: GmlQualifiedVolume = de::from_str(&xml).expect("should work");
        let round_tripped = QualifiedVolume::from(round_tripped_gml);

        assert_eq!(qualified_volume, round_tripped);
    }
}
