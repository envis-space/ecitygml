use ecitygml_core::model::core::{AsAbstractFeature, AsAbstractSpace};
use ecitygml_core::operations::{CityModelGeometryIndex, Visitable};
use ecitygml_io::GmlReader;
use egml::model::base::Id;

#[test]
fn test_lod2_building_model_fzk() {
    let city_model =
        GmlReader::from_path("../../tests/fixtures/FZK-Haus-LoD2-KIT-IAI-KHH-B36-V1__v3.gml")
            .expect("should work")
            .finish()
            .expect("should work");

    let mut city_model_geometry_index = CityModelGeometryIndex::from_city_model(city_model);

    assert_eq!(city_model_geometry_index.objects_len(), 8);

    let id: Id = "GML_5856d7ad-5e34-498a-817b-9544bfbb1475"
        .try_into()
        .expect("should work");
    let city_object = city_model_geometry_index.get(&id);

    assert!(city_object.is_some());
    let envelope = city_object.unwrap().envelope();
    assert!(envelope.is_some());
}
