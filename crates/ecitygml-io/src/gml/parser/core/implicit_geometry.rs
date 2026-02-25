use crate::Error;
use crate::gml::parser::util::deserialize_space_separated_f64;
use ecitygml_core::model::core::ImplicitGeometry;
use egml::io::primitives::GmlPointProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn parse_implicit_geometry(xml_document: &[u8]) -> Result<ImplicitGeometry, Error> {
    let gml_implicit_geometry: GmlImplicitGeometry = de::from_reader(xml_document)?;
    let implicit_geometry: ImplicitGeometry = gml_implicit_geometry.try_into()?;

    Ok(implicit_geometry)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlImplicitGeometry {
    #[serde(
        rename = "transformationMatrix",
        deserialize_with = "deserialize_space_separated_f64"
    )]
    pub transformation_matrix: Vec<f64>,

    #[serde(rename = "referencePoint")]
    pub reference_point: GmlPointProperty,
}

impl TryFrom<GmlImplicitGeometry> for ImplicitGeometry {
    type Error = Error;

    fn try_from(gml_implicit_geometry: GmlImplicitGeometry) -> Result<Self, Self::Error> {
        Ok(ImplicitGeometry {
            reference_point: gml_implicit_geometry.reference_point.point.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::gml::parser::core::implicit_geometry::parse_implicit_geometry;

    #[test]
    fn test_parse_implicit_geometry_basic() {
        let xml_document = b"<ImplicitGeometry>
    <transformationMatrix>-0.5894514707536183 -0.8078037903020735 0.0 0.0 0.8078037903020735 -0.5894514707536183 0.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 0.0 1.0</transformationMatrix>
    <referencePoint>
        <gml:Point>
            <gml:pos srsDimension=\"3\">678298.3706294019 5403791.857383491 366.9430094360463</gml:pos>
        </gml:Point>
    </referencePoint>
</ImplicitGeometry>";

        let implicit_geometry = parse_implicit_geometry(xml_document).expect("should work");

        assert_eq!(
            implicit_geometry.reference_point.pos().x(),
            678298.3706294019
        );
        assert_eq!(
            implicit_geometry.reference_point.pos().y(),
            5403791.857383491
        );
        assert_eq!(
            implicit_geometry.reference_point.pos().z(),
            366.9430094360463
        );
    }
}
