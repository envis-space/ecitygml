"""Tests for FeatureType and LevelOfDetail enums."""

from ecitygml import Building, FeatureType, GmlReader, LevelOfDetail

from conftest import ASAM_JUNCTION, FZK_LOD2


class TestFeatureType:
    def test_equality(self) -> None:
        assert FeatureType.Building == FeatureType.Building

    def test_inequality(self) -> None:
        assert FeatureType.Road != FeatureType.Building

    def test_repr_contains_variant_name(self) -> None:
        assert "Building" in repr(FeatureType.Building)

    def test_matches_building_feature_type(self) -> None:
        b = GmlReader(FZK_LOD2).finish().buildings()[0]
        assert b.feature_type == FeatureType.Building

    def test_matches_road_feature_type(self) -> None:
        r = GmlReader(ASAM_JUNCTION).finish().roads()[0]
        assert r.feature_type == FeatureType.Road

    def test_all_expected_variants_exist(self) -> None:
        expected = [
            "Building",
            "Road",
            "Section",
            "Intersection",
            "TrafficSpace",
            "TrafficArea",
            "AuxiliaryTrafficSpace",
            "AuxiliaryTrafficArea",
            "WallSurface",
            "RoofSurface",
            "GroundSurface",
            "DoorSurface",
            "WindowSurface",
            "TinRelief",
            "ReliefFeature",
            "SolitaryVegetationObject",
            "CityFurniture",
        ]
        for name in expected:
            assert hasattr(FeatureType, name), f"Missing FeatureType.{name}"


class TestLevelOfDetail:
    def test_value_zero(self) -> None:
        assert LevelOfDetail.Zero.value == 0

    def test_value_one(self) -> None:
        assert LevelOfDetail.One.value == 1

    def test_value_two(self) -> None:
        assert LevelOfDetail.Two.value == 2

    def test_value_three(self) -> None:
        assert LevelOfDetail.Three.value == 3

    def test_equality(self) -> None:
        assert LevelOfDetail.One == LevelOfDetail.One

    def test_inequality(self) -> None:
        assert LevelOfDetail.One != LevelOfDetail.Two

    def test_repr_contains_variant_name(self) -> None:
        assert "One" in repr(LevelOfDetail.One)
