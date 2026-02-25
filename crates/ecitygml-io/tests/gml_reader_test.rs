use ecitygml_core::model::core::{AsAbstractFeature, AsAbstractSpace};
use ecitygml_io::GmlReader;

#[test]
fn test_lod1_building_model_fzk() {
    let city_model =
        GmlReader::from_path("../../tests/fixtures/FZK-Haus-LoD1-KIT-IAI-KHH-B36-V1__v3.gml")
            .expect("should work")
            .finish()
            .expect("should work");

    assert_eq!(city_model.city_objects_len(), 1);
    assert_eq!(city_model.buildings().len(), 1);

    let buildings = city_model.buildings();
    let building = buildings.first().expect("should work");
    assert_eq!(
        building.id().to_string(),
        "UUID_d281adfc-4901-0f52-540b-4cc1a9325f82"
    );

    assert!(building.lod1_solid().is_some());
}

#[test]
fn test_lod2_building_model_fzk() {
    let city_model =
        GmlReader::from_path("../../tests/fixtures/FZK-Haus-LoD2-KIT-IAI-KHH-B36-V1__v3.gml")
            .expect("should work")
            .finish()
            .expect("should work");

    assert_eq!(city_model.city_objects_len(), 1);
    assert_eq!(city_model.buildings().len(), 1);

    let buildings = city_model.buildings();
    let building = buildings.first().expect("should work");
    assert_eq!(
        building.id().to_string(),
        "UUID_d281adfc-4901-0f52-540b-4cc1a9325f82"
    );

    // assert!(building.lod2_solid().is_some());
    assert_eq!(building.wall_surface.len(), 4);
    assert_eq!(building.roof_surface.len(), 2);
    assert_eq!(building.ground_surface.len(), 1);
}

#[test]
fn test_lod3_building_model_fzk() {
    let city_model =
        GmlReader::from_path("../../tests/fixtures/FZK-Haus-LoD3-KIT-IAI-KHH-B36-V1__v3.gml")
            .expect("should work")
            .finish()
            .expect("should work");

    assert_eq!(city_model.city_objects_len(), 1);
    assert_eq!(city_model.buildings().len(), 1);

    let buildings = city_model.buildings();
    let building = buildings.first().expect("should work");
    assert_eq!(
        building.id().to_string(),
        "UUID_d281adfc-4901-0f52-540b-4cc1a9325f82"
    );

    assert_eq!(building.wall_surface.len(), 4);
    assert_eq!(building.roof_surface.len(), 2);
    assert_eq!(building.ground_surface.len(), 1);

    let wall_surface = building.wall_surface.first().expect("should work");
    assert_eq!(wall_surface.door_surface.len(), 1);
}

#[test]
fn test_lod2_building_model_tum() {
    let city_model = GmlReader::from_path("../../tests/fixtures/TUM-Main-Entrance.gml")
        .expect("should work")
        .finish()
        .expect("should work");

    assert_eq!(city_model.city_objects_len(), 1);

    let buildings = city_model.buildings();
    let building = buildings.first().expect("should work");
    assert_eq!(building.id().to_string(), "DEBY_LOD2_4959457");

    assert_eq!(building.wall_surface.len(), 14);
    assert_eq!(building.roof_surface.len(), 2);
    assert_eq!(building.ground_surface.len(), 1);
}

#[test]
fn test_road_model_asam_junction() {
    let city_model =
        GmlReader::from_path("../../tests/fixtures/ASAM-Ex_Bidirectional_Junction.gml")
            .expect("should work")
            .finish()
            .expect("should work");

    assert_eq!(city_model.city_objects_len(), 1);
    assert_eq!(city_model.roads().len(), 1);

    let roads = city_model.roads();
    let road = roads.first().expect("should work");
    assert_eq!(road.section.len(), 3);
    assert_eq!(road.intersection.len(), 1);

    let intersection = road.intersection.first().expect("should work");
    assert_eq!(intersection.traffic_space.len(), 3);
}

#[test]
fn test_road_model_asam_road_shape() {
    let city_model = GmlReader::from_path("../../tests/fixtures/ASAM-UC_RoadShape.gml")
        .expect("should work")
        .finish()
        .expect("should work");

    assert_eq!(city_model.city_objects_len(), 1);
    assert_eq!(city_model.roads().len(), 1);

    let roads = city_model.roads();
    let road = roads.first().expect("should work");
    assert_eq!(road.section.len(), 1);

    let section = road.section.first().expect("should work");
    assert_eq!(section.traffic_space.len(), 2);
    assert_eq!(section.auxiliary_traffic_space.len(), 2);
}
