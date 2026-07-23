use ecitygml_core::model::building::Building;
use ecitygml_core::model::common::IterFeatures;
use ecitygml_core::model::construction::{
    AbstractConstructionSurfaceKind, RoofSurface, WallSurface,
};
use ecitygml_core::model::construction::{
    AbstractFillingSurfaceKind, AsAbstractConstructionSurface, DoorSurface, GroundSurface,
    WindowSurface,
};
use ecitygml_core::model::core::enums::RelativeToTerrain;
use ecitygml_core::model::core::{
    AbstractSpaceBoundaryKind, AbstractThematicSurfaceKind, AsAbstractCityObject,
    AsAbstractFeature, AsAbstractSpace,
};
use ecitygml_core::model::transportation::{AsAbstractTransportationSpace, Road};
use ecitygml_io::GmlReader;

fn collect_wall_surfaces(building: &ecitygml_core::model::building::Building) -> Vec<&WallSurface> {
    building
        .boundaries()
        .iter()
        .flat_map(|x| x.object())
        .filter_map(|x| match x {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(
                AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(
                    AbstractConstructionSurfaceKind::WallSurface(w),
                ),
            ) => Some(w),
            _ => None,
        })
        .collect()
}

fn collect_roof_surfaces(building: &ecitygml_core::model::building::Building) -> Vec<&RoofSurface> {
    building
        .boundaries()
        .iter()
        .flat_map(|x| x.object())
        .filter_map(|x| match x {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(
                AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(
                    AbstractConstructionSurfaceKind::RoofSurface(r),
                ),
            ) => Some(r),
            _ => None,
        })
        .collect()
}

fn collect_ground_surfaces(
    building: &ecitygml_core::model::building::Building,
) -> Vec<&GroundSurface> {
    building
        .boundaries()
        .iter()
        .flat_map(|x| x.object())
        .filter_map(|x| match x {
            AbstractSpaceBoundaryKind::AbstractThematicSurfaceKind(
                AbstractThematicSurfaceKind::AbstractConstructionSurfaceKind(
                    AbstractConstructionSurfaceKind::GroundSurface(g),
                ),
            ) => Some(g),
            _ => None,
        })
        .collect()
}

#[test]
fn test_lod1_building_model_fzk() {
    let city_model =
        GmlReader::from_path("../../tests/fixtures/FZK-Haus-LoD1-KIT-IAI-KHH-B36-V1__v3.gml")
            .expect("should work")
            .finish()
            .expect("should work");

    assert_eq!(city_model.city_object_members_len(), 1);

    let buildings: Vec<&Building> = city_model
        .iter_features()
        .flat_map(|x| x.try_into())
        .collect();
    assert_eq!(buildings.len(), 1);
    let building = buildings.first().expect("should work");
    assert_eq!(
        building.feature_id().to_string(),
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

    assert_eq!(city_model.city_object_members_len(), 1);

    let buildings: Vec<&Building> = city_model
        .iter_features()
        .flat_map(|x| x.try_into())
        .collect();
    assert_eq!(buildings.len(), 1);
    let building = buildings.first().expect("should work");
    assert_eq!(
        building.feature_id().to_string(),
        "UUID_d281adfc-4901-0f52-540b-4cc1a9325f82"
    );

    assert_eq!(collect_wall_surfaces(building).len(), 4);
    assert_eq!(collect_roof_surfaces(building).len(), 2);
    assert_eq!(collect_ground_surfaces(building).len(), 1);
}

#[test]
fn test_lod3_building_model_fzk() {
    let city_model =
        GmlReader::from_path("../../tests/fixtures/FZK-Haus-LoD3-KIT-IAI-KHH-B36-V1__v3.gml")
            .expect("should work")
            .finish()
            .expect("should work");

    assert_eq!(city_model.city_object_members_len(), 1);
    let buildings: Vec<&Building> = city_model
        .iter_features()
        .flat_map(|x| x.try_into())
        .collect();
    assert_eq!(buildings.len(), 1);

    let building = buildings.first().expect("should work");
    assert_eq!(
        building.feature_id().to_string(),
        "UUID_d281adfc-4901-0f52-540b-4cc1a9325f82"
    );
    assert_eq!(
        building.relative_to_terrain().expect("should work"),
        RelativeToTerrain::EntirelyAboveTerrain
    );

    let wall_surfaces = collect_wall_surfaces(building);
    assert_eq!(wall_surfaces.len(), 4);
    assert_eq!(collect_roof_surfaces(building).len(), 2);
    assert_eq!(collect_ground_surfaces(building).len(), 1);

    let wall_surface = wall_surfaces.first().expect("should work");
    let door_surfaces: Vec<&DoorSurface> = wall_surface
        .filling_surfaces()
        .iter()
        .flat_map(|x| x.object())
        .filter_map(|x| match x {
            AbstractFillingSurfaceKind::DoorSurface(x) => Some(x),
            _ => None,
        })
        .collect();
    assert_eq!(door_surfaces.len(), 1);
}

#[test]
fn test_lod2_building_model_tum() {
    let city_model = GmlReader::from_path("../../tests/fixtures/TUM-Main-Entrance.gml")
        .expect("should work")
        .finish()
        .expect("should work");

    assert_eq!(city_model.city_object_members_len(), 1);

    let buildings: Vec<&Building> = city_model
        .iter_features()
        .flat_map(|x| x.try_into())
        .collect();
    let building = buildings.first().expect("should work");
    assert_eq!(building.feature_id().to_string(), "DEBY_LOD2_4959457");

    assert_eq!(collect_wall_surfaces(building).len(), 14);
    assert_eq!(collect_roof_surfaces(building).len(), 2);
    assert_eq!(collect_ground_surfaces(building).len(), 1);
}

#[test]
fn test_road_model_asam_junction() {
    let city_model =
        GmlReader::from_path("../../tests/fixtures/ASAM-Ex_Bidirectional_Junction.gml")
            .expect("should work")
            .finish()
            .expect("should work");

    assert_eq!(city_model.city_object_members_len(), 1);

    let roads: Vec<&Road> = city_model
        .iter_features()
        .flat_map(|x| x.try_into())
        .collect();
    assert_eq!(roads.len(), 1);
    let road = roads.first().expect("should work");
    assert_eq!(road.sections().len(), 3);
    assert_eq!(road.intersections().len(), 1);

    let intersection = road
        .intersections()
        .first()
        .expect("should work")
        .object()
        .unwrap();
    assert_eq!(intersection.traffic_spaces().len(), 3);
}

#[test]
fn test_road_model_asam_road_shape() {
    let city_model = GmlReader::from_path("../../tests/fixtures/ASAM-UC_RoadShape.gml")
        .expect("should work")
        .finish()
        .expect("should work");

    assert_eq!(city_model.city_object_members_len(), 1);

    let roads: Vec<&Road> = city_model
        .iter_features()
        .flat_map(|x| x.try_into())
        .collect();
    assert_eq!(roads.len(), 1);
    let road = roads.first().expect("should work");
    assert_eq!(road.sections().len(), 1);

    let section = road
        .sections()
        .first()
        .expect("should work")
        .object()
        .unwrap();
    assert_eq!(section.traffic_spaces().len(), 2);
    assert_eq!(section.auxiliary_traffic_spaces().len(), 2);
}
