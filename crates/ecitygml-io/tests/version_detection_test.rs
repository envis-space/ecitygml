use ecitygml_io::{Error, GmlReader};
use std::io::Cursor;

fn read(xml: &str) -> Result<ecitygml_core::model::core::CityModel, Error> {
    GmlReader::new(
        Cursor::new(xml.as_bytes().to_vec()),
        ecitygml_io::CitygmlFormat::Gml,
    )
    .finish()
}

#[test]
fn test_rejects_citygml_2_0() {
    let xml = r#"<?xml version="1.0"?>
        <CityModel xmlns="http://www.opengis.net/citygml/2.0"
                   xmlns:gml="http://www.opengis.net/gml">
        </CityModel>"#;

    let err = read(xml).expect_err("CityGML 2.0 should be rejected");
    assert!(matches!(err, Error::UnsupportedCityGmlVersion(_)));
}

#[test]
fn test_rejects_unknown_namespace() {
    let xml = r#"<?xml version="1.0"?>
        <root xmlns="http://example.com/not-citygml"></root>"#;

    let err = read(xml).expect_err("non-CityGML input should be rejected");
    assert!(matches!(err, Error::UnknownCityGmlVersion));
}

#[test]
fn test_accepts_citygml_3_0() {
    let xml = r#"<?xml version="1.0"?>
        <CityModel xmlns="http://www.opengis.net/citygml/3.0"
                   xmlns:gml="http://www.opengis.net/gml/3.2">
        </CityModel>"#;

    read(xml).expect("well-formed CityGML 3.0 should pass the version check");
}
