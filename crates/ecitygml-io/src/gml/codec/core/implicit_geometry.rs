use crate::Error;
use crate::gml::codec::core::abstract_appearance_property::{
    deserialize_abstract_appearance_property, serialize_abstract_appearance_property,
};
use crate::gml::codec::core::transformation_matrix_4x4::GmlTransformationMatrix4x4;
use crate::gml::util::{CityGmlElement, collect_gml_child};
use ecitygml_core::model::core::ImplicitGeometry;
use ecitygml_core::model::core::values::MimeTypeValue;
use egml::io::codec::basic::GmlCode;
use egml::io::codec::geometry::primitives::{deserialize_point_property, serialize_point_property};
use egml::io::codec::geometry::{
    deserialize_abstract_geometry_property, serialize_abstract_geometry_property,
};
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, XmlNodeParts, collect_children, extract_xml_element_spans,
    serialize_inner,
};
use egml::model::base::Id;
use egml::model::basic_types::Code;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);
fn generate_object_id() -> Id {
    let n = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    Id::try_from(format!("ecitygml-gen-object-{n}"))
        .expect("ecitygml-gen-object-{n} is always a valid xsd:ID")
}

pub fn deserialize_implicit_geometry(xml_document: &[u8]) -> Result<ImplicitGeometry, Error> {
    let spans = extract_xml_element_spans(xml_document)?;

    let mut parsed_result = None;
    let mut reference_point_result = None;
    let mut relative_geometry_result = None;
    let mut appearances_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            parsed_result = Some(
                quick_xml::de::from_reader::<_, GmlImplicitGeometry>(xml_document)
                    .map_err(Error::from),
            );
        });
        s.spawn(|_| {
            reference_point_result = Some(collect_gml_child(
                xml_document,
                &spans,
                CityGmlElement::ReferencePointProperty.into(),
                deserialize_point_property,
            ));
        });
        s.spawn(|_| {
            relative_geometry_result = Some(collect_gml_child(
                xml_document,
                &spans,
                CityGmlElement::RelativeGeometryProperty.into(),
                deserialize_abstract_geometry_property,
            ));
        });
        s.spawn(|_| {
            appearances_result = Some(collect_children(
                xml_document,
                &spans,
                CityGmlElement::AbstractAppearanceProperty.into(),
                deserialize_abstract_appearance_property,
            ));
        });
    });

    let parsed = parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let reference_point = reference_point_result
        .expect("rayon::scope guarantees all spawns complete")?
        .ok_or(Error::ElementNotFound("reference point not found".into()))?;
    let relative_geometry =
        relative_geometry_result.expect("rayon::scope guarantees all spawns complete")?;
    let appearances = appearances_result.expect("rayon::scope guarantees all spawns complete")?;

    let object_id: Id = if let Some(object_id) = parsed.object_id {
        Id::try_from(object_id)?
    } else {
        generate_object_id()
    };

    let mut implicit_geometry: ImplicitGeometry = ImplicitGeometry::new(
        object_id,
        parsed.transformation_matrix.try_into()?,
        reference_point,
    );
    implicit_geometry.set_mime_type_opt(parsed.mime_type.map(Code::from).map(MimeTypeValue::from));
    implicit_geometry.set_library_object_opt(parsed.library_object);
    implicit_geometry.set_relative_geometry_opt(relative_geometry);
    implicit_geometry.set_appearances(appearances);

    Ok(implicit_geometry)
}

pub fn serialize_implicit_geometry(
    implicit_geometry: &ImplicitGeometry,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();

    if let Some(raw) = serialize_inner(GmlImplicitGeometry::from(implicit_geometry), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    xml_node_parts.content.push(
        serialize_point_property(
            implicit_geometry.reference_point(),
            formatting,
            CityGmlElement::ReferencePointProperty.into(),
        )
        .map_err(Error::from)?
        .into(),
    );

    if let Some(prop) = implicit_geometry.relative_geometry() {
        xml_node_parts.content.push(
            serialize_abstract_geometry_property(
                prop,
                formatting,
                CityGmlElement::RelativeGeometryProperty.into(),
            )
            .map_err(Error::from)?
            .into(),
        );
    }

    for prop in implicit_geometry.appearances() {
        xml_node_parts.content.push(XmlNodeContent::Child(
            serialize_abstract_appearance_property(prop, formatting)?,
        ));
    }

    Ok(XmlNode::new(
        CityGmlElement::ImplicitGeometry.into(),
        xml_node_parts,
    ))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlImplicitGeometry {
    #[serde(rename = "objectId", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    #[serde(rename = "transformationMatrix")]
    pub transformation_matrix: GmlTransformationMatrix4x4,

    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<GmlCode>,

    #[serde(rename = "libraryObject", skip_serializing_if = "Option::is_none")]
    pub library_object: Option<String>,
}

impl From<&ImplicitGeometry> for GmlImplicitGeometry {
    fn from(item: &ImplicitGeometry) -> Self {
        GmlImplicitGeometry {
            object_id: Some(item.object_id().clone().into()),
            transformation_matrix: (*item.transformation_matrix()).into(),
            mime_type: item.mime_type().map(MimeTypeValue::code).map(Into::into),
            library_object: item.library_object().map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gml::codec::core::implicit_geometry::{
        deserialize_implicit_geometry, serialize_implicit_geometry,
    };
    use egml::io::util::Formatting;

    #[test]
    fn test_deserialize_implicit_geometry_basic() {
        let xml_document = b"<ImplicitGeometry>
    <transformationMatrix>-0.5894514707536183 -0.8078037903020735 0.0 0.0 0.8078037903020735 -0.5894514707536183 0.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 0.0 1.0</transformationMatrix>
    <referencePoint>
        <gml:Point>
            <gml:pos srsDimension=\"3\">678298.3706294019 5403791.857383491 366.9430094360463</gml:pos>
        </gml:Point>
    </referencePoint>
</ImplicitGeometry>";

        let implicit_geometry = deserialize_implicit_geometry(xml_document).expect("should work");

        assert_eq!(
            implicit_geometry
                .reference_point()
                .object()
                .expect("must exist")
                .pos()
                .x(),
            678298.3706294019
        );
        assert_eq!(
            implicit_geometry
                .reference_point()
                .object()
                .expect("must exist")
                .pos()
                .y(),
            5403791.857383491
        );
        assert_eq!(
            implicit_geometry
                .reference_point()
                .object()
                .expect("must exist")
                .pos()
                .z(),
            366.9430094360463
        );
    }

    #[test]
    fn test_serialize_implicit_geometry_reference_point() {
        let xml_document = b"<ImplicitGeometry>
    <transformationMatrix>-0.5894514707536183 -0.8078037903020735 0.0 0.0 0.8078037903020735 -0.5894514707536183 0.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 0.0 1.0</transformationMatrix>
    <referencePoint>
        <gml:Point>
            <gml:pos srsDimension=\"3\">678298.3706294019 5403791.857383491 366.9430094360463</gml:pos>
        </gml:Point>
    </referencePoint>
</ImplicitGeometry>";

        let implicit_geometry =
            deserialize_implicit_geometry(xml_document).expect("should deserialize");
        let xml = serialize_implicit_geometry(&implicit_geometry, Formatting::Compact)
            .expect("should serialize")
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        assert!(xml.contains("referencePoint"), "missing <referencePoint>");
        assert!(
            xml.contains("678298.3706294019"),
            "missing reference point x coordinate"
        );
        assert!(
            xml.contains("5403791.857383491"),
            "missing reference point y coordinate"
        );
        assert!(
            xml.contains("366.9430094360463"),
            "missing reference point z coordinate"
        );
    }

    #[test]
    fn test_deserialize_implicit_geometry_with_relative_geometry() {
        let xml_document = b"<ImplicitGeometry>
  <transformationMatrix>10.289999999999994 0.0 0.0 0.0 0.0 10.289999999999994 0.0 0.0 0.0 0.0 10.289999999999994 0.0 0.0 0.0 0.0 1.0</transformationMatrix>
  <referencePoint>
    <gml:Point>
      <gml:pos>689949.3 5324001.1 586.3806</gml:pos>
    </gml:Point>
  </referencePoint>
  <relativeGeometry>
    <gml:MultiGeometry srsName=\"urn:ogc:def:crs:EPSG::25832\" srsDimension=\"3\" gml:id=\"fme-gen-1b18fb8f-dbee-460c-bd8e-4e3229066ea2\">
      <gml:geometryMember>
        <gml:MultiGeometry>
          <gml:geometryMember>
            <gml:Polygon>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>1.1102230246251565E-16 -0.08104761325536736 0.40000000000000024 5.551115123125783E-17 -0.25873011317535366 0.42868290603009473 -0.18294981752345554 -0.1829498175234554 0.42868290603009473 -0.057309316931854926 -0.057309316931854926 0.40000000000000024 1.1102230246251565E-16 -0.08104761325536736 0.40000000000000024</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Polygon>
          </gml:geometryMember>
          <gml:geometryMember>
            <gml:Polygon>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>-0.057309316931854926 -0.057309316931854926 0.40000000000000024 -0.18294981752345554 -0.1829498175234554 0.42868290603009473 -0.25873011317535366 0.0 0.42868290603009473 -0.0810476132553673 0.0 0.40000000000000024 -0.057309316931854926 -0.057309316931854926 0.40000000000000024</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Polygon>
          </gml:geometryMember>
          <gml:geometryMember>
            <gml:Polygon>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>-0.24026196252347953 0.24026196252347976 0.49950174129794134 -0.24748737341529176 0.2474873734152918 0.6403170178482882 5.551115123125783E-17 0.3500000000000002 0.6403170178482882 5.551115123125783E-17 0.3397817259230812 0.49950174129794134 -0.24026196252347953 0.24026196252347976 0.49950174129794134</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Polygon>
          </gml:geometryMember>
        </gml:MultiGeometry>
      </gml:geometryMember>
    </gml:MultiGeometry>
  </relativeGeometry>
</ImplicitGeometry>";

        let implicit_geometry =
            deserialize_implicit_geometry(xml_document).expect("should deserialize");

        let reference_point = implicit_geometry
            .reference_point()
            .object()
            .expect("reference point must exist");
        assert_eq!(reference_point.pos().x(), 689949.3);
        assert_eq!(reference_point.pos().y(), 5324001.1);
        assert_eq!(reference_point.pos().z(), 586.3806);

        assert_eq!(
            implicit_geometry.transformation_matrix().matrix()[(0, 0)],
            10.289999999999994
        );

        assert!(
            implicit_geometry.relative_geometry().is_some(),
            "relative geometry must be present"
        );
    }

    #[test]
    fn test_serialize_implicit_geometry_with_relative_geometry() {
        let xml_document = b"<ImplicitGeometry>
  <transformationMatrix>10.289999999999994 0.0 0.0 0.0 0.0 10.289999999999994 0.0 0.0 0.0 0.0 10.289999999999994 0.0 0.0 0.0 0.0 1.0</transformationMatrix>
  <referencePoint>
    <gml:Point>
      <gml:pos>689949.3 5324001.1 586.3806</gml:pos>
    </gml:Point>
  </referencePoint>
  <relativeGeometry>
    <gml:MultiGeometry srsName=\"urn:ogc:def:crs:EPSG::25832\" srsDimension=\"3\" gml:id=\"fme-gen-1b18fb8f-dbee-460c-bd8e-4e3229066ea2\">
      <gml:geometryMember>
        <gml:MultiGeometry>
          <gml:geometryMember>
            <gml:Polygon>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>1.1102230246251565E-16 -0.08104761325536736 0.40000000000000024 5.551115123125783E-17 -0.25873011317535366 0.42868290603009473 -0.18294981752345554 -0.1829498175234554 0.42868290603009473 -0.057309316931854926 -0.057309316931854926 0.40000000000000024 1.1102230246251565E-16 -0.08104761325536736 0.40000000000000024</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Polygon>
          </gml:geometryMember>
          <gml:geometryMember>
            <gml:Polygon>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>-0.057309316931854926 -0.057309316931854926 0.40000000000000024 -0.18294981752345554 -0.1829498175234554 0.42868290603009473 -0.25873011317535366 0.0 0.42868290603009473 -0.0810476132553673 0.0 0.40000000000000024 -0.057309316931854926 -0.057309316931854926 0.40000000000000024</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Polygon>
          </gml:geometryMember>
          <gml:geometryMember>
            <gml:Polygon>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>-0.24026196252347953 0.24026196252347976 0.49950174129794134 -0.24748737341529176 0.2474873734152918 0.6403170178482882 5.551115123125783E-17 0.3500000000000002 0.6403170178482882 5.551115123125783E-17 0.3397817259230812 0.49950174129794134 -0.24026196252347953 0.24026196252347976 0.49950174129794134</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Polygon>
          </gml:geometryMember>
        </gml:MultiGeometry>
      </gml:geometryMember>
    </gml:MultiGeometry>
  </relativeGeometry>
</ImplicitGeometry>";

        let implicit_geometry =
            deserialize_implicit_geometry(xml_document).expect("should deserialize");
        let xml = serialize_implicit_geometry(&implicit_geometry, Formatting::Compact)
            .expect("should serialize")
            .to_string(Formatting::Compact)
            .expect("should convert to string");

        assert!(xml.contains("referencePoint"), "missing <referencePoint>");
        assert!(xml.contains("689949.3"), "missing reference point x");
        assert!(xml.contains("5324001.1"), "missing reference point y");
        assert!(xml.contains("586.3806"), "missing reference point z");
        assert!(
            xml.contains("relativeGeometry"),
            "missing <relativeGeometry>"
        );
        assert!(xml.contains("MultiGeometry"), "missing <MultiGeometry>");
        assert!(xml.contains("Polygon"), "missing <Polygon>");
    }
}
