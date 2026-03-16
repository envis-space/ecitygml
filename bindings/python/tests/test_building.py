"""Tests for Building and its thematic surfaces across LoD1, LoD2, LoD3, and
the TUM Main Entrance dataset."""

import pytest

from ecitygml import (
    Building,
    CityObjectClass,
    DoorSurface,
    Envelope,
    GroundSurface,
    MultiSurface,
    RoofSurface,
    Solid,
    WallSurface,
    WindowSurface,
)


class TestBuildingLoD1:
    def test_id(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.id == "UUID_d281adfc-4901-0f52-540b-4cc1a9325f82"

    def test_city_object_class(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.city_object_class == CityObjectClass.Building

    def test_storeys_above_ground(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.storeys_above_ground == 2

    def test_storeys_below_ground(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.storeys_below_ground == 0

    def test_bounded_by_lower_corner(self, fzk_lod1_building: Building) -> None:
        env = fzk_lod1_building.bounded_by
        assert isinstance(env, Envelope)
        assert env.lower_corner == pytest.approx((457842.0, 5439083.0, 111.8))

    def test_lod1_solid_present(self, fzk_lod1_building: Building) -> None:
        assert isinstance(fzk_lod1_building.lod1_solid, Solid)

    def test_lod2_solid_absent(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.lod2_solid is None

    def test_lod0_multi_surface_absent(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.lod0_multi_surface is None

    def test_no_thematic_surfaces(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.wall_surface == []
        assert fzk_lod1_building.roof_surface == []
        assert fzk_lod1_building.ground_surface == []

    def test_repr_contains_id(self, fzk_lod1_building: Building) -> None:
        assert "UUID_d281adfc" in repr(fzk_lod1_building)


class TestBuildingLoD2:
    def test_wall_surface_count(self, fzk_lod2_building: Building) -> None:
        assert len(fzk_lod2_building.wall_surface) == 4

    def test_roof_surface_count(self, fzk_lod2_building: Building) -> None:
        assert len(fzk_lod2_building.roof_surface) == 2

    def test_ground_surface_count(self, fzk_lod2_building: Building) -> None:
        assert len(fzk_lod2_building.ground_surface) == 1

    def test_lod1_solid_absent(self, fzk_lod2_building: Building) -> None:
        assert fzk_lod2_building.lod1_solid is None

    def test_wall_surface_type(self, fzk_lod2_building: Building) -> None:
        assert isinstance(fzk_lod2_building.wall_surface[0], WallSurface)

    def test_roof_surface_type(self, fzk_lod2_building: Building) -> None:
        assert isinstance(fzk_lod2_building.roof_surface[0], RoofSurface)

    def test_ground_surface_type(self, fzk_lod2_building: Building) -> None:
        assert isinstance(fzk_lod2_building.ground_surface[0], GroundSurface)

    def test_wall_has_id(self, fzk_lod2_building: Building) -> None:
        assert fzk_lod2_building.wall_surface[0].id != ""

    def test_wall_lod2_multi_surface(self, fzk_lod2_building: Building) -> None:
        assert isinstance(fzk_lod2_building.wall_surface[0].lod2_multi_surface, MultiSurface)

    def test_wall_lod2_polygon_count(self, fzk_lod2_building: Building) -> None:
        ms = fzk_lod2_building.wall_surface[0].lod2_multi_surface
        assert len(ms.polygons()) == 1

    def test_wall_lod0_multi_surface_absent(self, fzk_lod2_building: Building) -> None:
        assert fzk_lod2_building.wall_surface[0].lod0_multi_surface is None

    def test_wall_lod2_multi_surface_can_triangulate(self, fzk_lod2_building: Building) -> None:
        ms = fzk_lod2_building.wall_surface[0].lod2_multi_surface
        assert len(ms.triangulate().triangles) > 0

    def test_ground_polygon_exterior_point_count(self, fzk_lod2_building: Building) -> None:
        ms = fzk_lod2_building.ground_surface[0].lod2_multi_surface
        assert len(ms.polygons()[0].exterior.points) >= 4

    def test_wall_no_door_or_window_surfaces(self, fzk_lod2_building: Building) -> None:
        for ws in fzk_lod2_building.wall_surface:
            assert ws.door_surface == []
            assert ws.window_surface == []

    def test_envelope_volume(self, fzk_lod2_building: Building) -> None:
        assert fzk_lod2_building.bounded_by.volume == pytest.approx(782.12, abs=1.0)


class TestBuildingLoD3:
    def test_wall_surface_count(self, fzk_lod3_building: Building) -> None:
        assert len(fzk_lod3_building.wall_surface) == 4

    def test_first_wall_door_count(self, fzk_lod3_building: Building) -> None:
        assert len(fzk_lod3_building.wall_surface[0].door_surface) == 1

    def test_first_wall_window_count(self, fzk_lod3_building: Building) -> None:
        assert len(fzk_lod3_building.wall_surface[0].window_surface) == 3

    def test_door_surface_type(self, fzk_lod3_building: Building) -> None:
        door = fzk_lod3_building.wall_surface[0].door_surface[0]
        assert isinstance(door, DoorSurface)

    def test_window_surface_type(self, fzk_lod3_building: Building) -> None:
        win = fzk_lod3_building.wall_surface[0].window_surface[0]
        assert isinstance(win, WindowSurface)

    def test_door_has_id(self, fzk_lod3_building: Building) -> None:
        assert fzk_lod3_building.wall_surface[0].door_surface[0].id != ""

    def test_door_lod3_multi_surface(self, fzk_lod3_building: Building) -> None:
        door = fzk_lod3_building.wall_surface[0].door_surface[0]
        assert isinstance(door.lod3_multi_surface, MultiSurface)

    def test_window_lod3_multi_surface(self, fzk_lod3_building: Building) -> None:
        win = fzk_lod3_building.wall_surface[0].window_surface[0]
        assert isinstance(win.lod3_multi_surface, MultiSurface)

    def test_wall_lod3_multi_surface(self, fzk_lod3_building: Building) -> None:
        assert isinstance(fzk_lod3_building.wall_surface[0].lod3_multi_surface, MultiSurface)

    def test_total_door_count(self, fzk_lod3_building: Building) -> None:
        total = sum(len(ws.door_surface) for ws in fzk_lod3_building.wall_surface)
        assert total == 2

    def test_total_window_count(self, fzk_lod3_building: Building) -> None:
        total = sum(len(ws.window_surface) for ws in fzk_lod3_building.wall_surface)
        assert total == 11


class TestTumBuilding:
    def test_id(self, tum_building: Building) -> None:
        assert tum_building.id == "DEBY_LOD2_4959457"

    def test_city_object_class(self, tum_building: Building) -> None:
        assert tum_building.city_object_class == CityObjectClass.Building

    def test_wall_surface_count(self, tum_building: Building) -> None:
        assert len(tum_building.wall_surface) == 14

    def test_roof_surface_count(self, tum_building: Building) -> None:
        assert len(tum_building.roof_surface) == 2

    def test_ground_surface_count(self, tum_building: Building) -> None:
        assert len(tum_building.ground_surface) == 1

    def test_bounded_by_lower_corner_x(self, tum_building: Building) -> None:
        assert tum_building.bounded_by.lower_corner[0] == pytest.approx(691009.16, abs=0.1)

    def test_wall_lod2_multi_surface(self, tum_building: Building) -> None:
        ws = tum_building.wall_surface[0]
        assert isinstance(ws.lod2_multi_surface, MultiSurface)
        assert len(ws.lod2_multi_surface.polygons()) >= 1

    def test_wall_can_triangulate(self, tum_building: Building) -> None:
        ms = tum_building.wall_surface[0].lod2_multi_surface
        assert len(ms.triangulate().triangles) > 0
