use ecitygml_core::arena::CityModelArena;
use ecitygml_io::GmlReader;
use egml::model::base::Id;
use egml::model::feature::AsAbstractFeature;

#[test]
fn test_lod2_building_model_fzk() {
    let city_model =
        GmlReader::from_path("../../tests/fixtures/FZK-Haus-LoD2-KIT-IAI-KHH-B36-V1__v3.gml")
            .expect("should work")
            .finish()
            .expect("should work");

    let city_model_arena = CityModelArena::from_city_model(city_model).expect("should work");

    assert_eq!(city_model_arena.iter_city_objects().count(), 8);

    let id: Id = "GML_5856d7ad-5e34-498a-817b-9544bfbb1475"
        .try_into()
        .expect("should work");
    let city_object_opt = city_model_arena.get_city_object_by_id(&id);

    assert!(city_object_opt.is_some());
    let city_object = city_object_opt.unwrap();
    let envelope = city_object.bounded_by().and_then(|x| x.envelope());
    assert!(envelope.is_some());
}
