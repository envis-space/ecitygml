use crate::Error;
use ecitygml_core::model::core::TransformationMatrix4x4;
use egml::io::util::serde_helpers::{
    deserialize_space_separated_f64, serialize_space_separated_f64,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTransformationMatrix4x4 {
    #[serde(
        rename = "$value",
        deserialize_with = "deserialize_space_separated_f64",
        serialize_with = "serialize_space_separated_f64"
    )]
    pub values: Vec<f64>,
}

impl TryFrom<GmlTransformationMatrix4x4> for TransformationMatrix4x4 {
    type Error = Error;

    fn try_from(item: GmlTransformationMatrix4x4) -> Result<Self, Self::Error> {
        Ok(TransformationMatrix4x4::try_from_row_major(item.values)?)
    }
}

impl From<TransformationMatrix4x4> for GmlTransformationMatrix4x4 {
    fn from(item: TransformationMatrix4x4) -> Self {
        Self {
            values: item.to_row_major(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de;
    use quick_xml::se::to_string;

    #[test]
    fn test_deserialize_transformation_matrix_4x4_basic() {
        let xml_document = b"<transformationMatrix>-0.5894514707536183 -0.8078037903020735 0.0 0.0 0.8078037903020735 -0.5894514707536183 0.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 0.0 1.0</transformationMatrix>";

        let gml: GmlTransformationMatrix4x4 =
            de::from_reader(xml_document.as_ref()).expect("should deserialize");
        assert_eq!(gml.values.len(), 16);
    }

    #[test]
    fn test_try_from_valid() {
        let values: Vec<f64> = vec![
            -0.589, -0.808, 0.0, 0.0, 0.808, -0.589, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            1.0,
        ];
        let gml = GmlTransformationMatrix4x4 { values };
        let matrix: TransformationMatrix4x4 = gml.try_into().expect("should convert");
        assert_eq!(matrix.matrix()[(0, 1)], -0.808);
    }

    #[test]
    fn test_try_from_wrong_count() {
        let gml = GmlTransformationMatrix4x4 {
            values: vec![1.0; 9],
        };
        let result = TransformationMatrix4x4::try_from(gml);
        assert!(result.is_err());
    }

    #[test]
    fn test_roundtrip() {
        let xml_document = b"<transformationMatrix>1.0 0.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 0.0 1.0</transformationMatrix>";
        let gml: GmlTransformationMatrix4x4 =
            de::from_reader(xml_document.as_ref()).expect("deserialize");
        let matrix: TransformationMatrix4x4 = gml.try_into().expect("convert");
        let gml2 = GmlTransformationMatrix4x4::from(matrix);
        let xml_out = to_string(&gml2).expect("serialize");
        assert!(xml_out.contains("1 0 0 0"));
    }
}
