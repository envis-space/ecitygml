"""Tests for transportation types: Road, Section, Intersection, TrafficSpace,
TrafficArea, AuxiliaryTrafficSpace, AuxiliaryTrafficArea, and MultiCurve."""

import pytest

from ecitygml import (
    AuxiliaryTrafficArea,
    AuxiliaryTrafficSpace,
    FeatureType,
    DirectPosition,
    Envelope,
    Intersection,
    MultiCurve,
    MultiSurface,
    Road,
    Section,
    TrafficArea,
    TrafficSpace,
)


class TestRoad:
    def test_type(self, asam_junction_road: Road) -> None:
        assert isinstance(asam_junction_road, Road)

    def test_id(self, asam_junction_road: Road) -> None:
        assert asam_junction_road.id == "UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f"

    def test_feature_type(self, asam_junction_road: Road) -> None:
        assert asam_junction_road.feature_type == FeatureType.Road

    def test_section_count(self, asam_junction_road: Road) -> None:
        assert len(asam_junction_road.section) == 3

    def test_intersection_count(self, asam_junction_road: Road) -> None:
        assert len(asam_junction_road.intersection) == 1

    def test_section_type(self, asam_junction_road: Road) -> None:
        assert isinstance(asam_junction_road.section[0], Section)

    def test_intersection_type(self, asam_junction_road: Road) -> None:
        assert isinstance(asam_junction_road.intersection[0], Intersection)

    def test_repr_contains_road(self, asam_junction_road: Road) -> None:
        assert "Road" in repr(asam_junction_road)


class TestSection:
    def test_id(self, asam_section: Section) -> None:
        assert asam_section.id == "UUID_0950bfa5-204e-33e6-bdb7-c5c318d73a29"

    def test_feature_type(self, asam_section: Section) -> None:
        assert asam_section.feature_type == FeatureType.Section

    def test_traffic_space_count(self, asam_section: Section) -> None:
        assert len(asam_section.traffic_space) == 2

    def test_traffic_space_type(self, asam_section: Section) -> None:
        assert isinstance(asam_section.traffic_space[0], TrafficSpace)

    def test_auxiliary_traffic_space_absent(self, asam_section: Section) -> None:
        assert asam_section.auxiliary_traffic_space == []

    def test_lod0_multi_curve_absent(self, asam_section: Section) -> None:
        assert asam_section.lod0_multi_curve is None

    def test_lod1_solid_absent(self, asam_section: Section) -> None:
        assert asam_section.lod1_solid is None


class TestIntersection:
    def test_id_non_empty(self, asam_junction_road: Road) -> None:
        assert asam_junction_road.intersection[0].id != ""

    def test_feature_type(self, asam_junction_road: Road) -> None:
        inter = asam_junction_road.intersection[0]
        assert inter.feature_type == FeatureType.Intersection

    def test_traffic_space_count(self, asam_junction_road: Road) -> None:
        assert len(asam_junction_road.intersection[0].traffic_space) == 3

    def test_traffic_space_type(self, asam_junction_road: Road) -> None:
        inter = asam_junction_road.intersection[0]
        assert isinstance(inter.traffic_space[0], TrafficSpace)


class TestTrafficSpace:
    def test_id_non_empty(self, asam_section: Section) -> None:
        assert asam_section.traffic_space[0].id != ""

    def test_feature_type(self, asam_section: Section) -> None:
        assert asam_section.traffic_space[0].feature_type == FeatureType.TrafficSpace

    def test_lod2_multi_curve_present(self, asam_section: Section) -> None:
        assert isinstance(asam_section.traffic_space[0].lod2_multi_curve, MultiCurve)

    def test_lod2_multi_surface_absent(self, asam_section: Section) -> None:
        assert asam_section.traffic_space[0].lod2_multi_surface is None

    def test_traffic_area_count(self, asam_section: Section) -> None:
        assert len(asam_section.traffic_space[0].traffic_area) == 1

    def test_traffic_area_type(self, asam_section: Section) -> None:
        assert isinstance(asam_section.traffic_space[0].traffic_area[0], TrafficArea)


class TestTrafficArea:
    def test_id(self, asam_section: Section) -> None:
        ta = asam_section.traffic_space[0].traffic_area[0]
        assert ta.id == "UUID_8625bf6f-57cb-3ddf-8fd1-6674b756a2e2"

    def test_feature_type(self, asam_section: Section) -> None:
        ta = asam_section.traffic_space[0].traffic_area[0]
        assert ta.feature_type == FeatureType.TrafficArea

    def test_lod2_multi_surface_present(self, asam_section: Section) -> None:
        ta = asam_section.traffic_space[0].traffic_area[0]
        assert isinstance(ta.lod2_multi_surface, MultiSurface)

    def test_lod2_multi_surface_polygon_count(self, asam_section: Section) -> None:
        ta = asam_section.traffic_space[0].traffic_area[0]
        assert len(ta.lod2_multi_surface.polygons()) == 8


class TestMultiCurve:
    def test_curves_returns_list(self, asam_section: Section) -> None:
        mc = asam_section.traffic_space[0].lod2_multi_curve
        assert isinstance(mc.curves(), list)

    def test_curve_count(self, asam_section: Section) -> None:
        assert len(asam_section.traffic_space[0].lod2_multi_curve.curves()) == 1

    def test_curve_points_are_direct_positions(self, asam_section: Section) -> None:
        for pt in asam_section.traffic_space[0].lod2_multi_curve.curves()[0]:
            assert isinstance(pt, DirectPosition)

    def test_curve_point_count(self, asam_section: Section) -> None:
        mc = asam_section.traffic_space[0].lod2_multi_curve
        assert len(mc.curves()[0]) == 9

    def test_compute_envelope_returns_envelope(self, asam_section: Section) -> None:
        mc = asam_section.traffic_space[0].lod2_multi_curve
        assert isinstance(mc.compute_envelope(), Envelope)

    def test_repr_is_non_empty(self, asam_section: Section) -> None:
        assert repr(asam_section.traffic_space[0].lod2_multi_curve)


class TestAuxiliaryTrafficSpace:
    def test_count(self, asam_road_section: Section) -> None:
        assert len(asam_road_section.auxiliary_traffic_space) == 2

    def test_type(self, asam_road_section: Section) -> None:
        assert isinstance(asam_road_section.auxiliary_traffic_space[0], AuxiliaryTrafficSpace)

    def test_feature_type(self, asam_road_section: Section) -> None:
        ats = asam_road_section.auxiliary_traffic_space[0]
        assert ats.feature_type == FeatureType.AuxiliaryTrafficSpace

    def test_id_non_empty(self, asam_road_section: Section) -> None:
        assert asam_road_section.auxiliary_traffic_space[0].id != ""

    def test_auxiliary_traffic_area_returns_list(self, asam_road_section: Section) -> None:
        ats = asam_road_section.auxiliary_traffic_space[0]
        assert isinstance(ats.auxiliary_traffic_area, list)

    def test_repr_contains_type_name(self, asam_road_section: Section) -> None:
        ats = asam_road_section.auxiliary_traffic_space[0]
        assert "AuxiliaryTrafficSpace" in repr(ats)


class TestAuxiliaryTrafficArea:
    def test_type(self, asam_road_section: Section) -> None:
        for ats in asam_road_section.auxiliary_traffic_space:
            for ata in ats.auxiliary_traffic_area:
                assert isinstance(ata, AuxiliaryTrafficArea)

    def test_feature_type(self, asam_road_section: Section) -> None:
        for ats in asam_road_section.auxiliary_traffic_space:
            for ata in ats.auxiliary_traffic_area:
                assert ata.feature_type == FeatureType.AuxiliaryTrafficArea

    def test_id_non_empty(self, asam_road_section: Section) -> None:
        for ats in asam_road_section.auxiliary_traffic_space:
            for ata in ats.auxiliary_traffic_area:
                assert ata.id != ""
