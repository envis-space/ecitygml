"""Tests for GmlReader and CityModel."""

import pytest

from ecitygml import CityModel, GmlReader

from conftest import ASAM_JUNCTION, FZK_LOD2


class TestGmlReader:
    def test_returns_city_model(self) -> None:
        model = GmlReader(FZK_LOD2).finish()
        assert isinstance(model, CityModel)

    def test_missing_file_raises_os_error(self) -> None:
        with pytest.raises(OSError):
            GmlReader("/nonexistent/path/file.gml").finish()

    def test_repr(self) -> None:
        assert repr(GmlReader(FZK_LOD2)) == "GmlReader"

    def test_with_rebuild_object_bounds(self) -> None:
        model = GmlReader(FZK_LOD2).with_rebuild_object_bounds(True).finish()
        assert model.city_objects_len() == 1


class TestCityModel:
    def test_city_objects_len_fzk(self) -> None:
        assert GmlReader(FZK_LOD2).finish().city_objects_len() == 1

    def test_city_objects_len_asam_junction(self) -> None:
        assert GmlReader(ASAM_JUNCTION).finish().city_objects_len() == 1

    def test_repr(self) -> None:
        assert repr(GmlReader(FZK_LOD2).finish()) == "CityModel(1 city objects)"

    def test_buildings_returns_list(self) -> None:
        buildings = GmlReader(FZK_LOD2).finish().buildings()
        assert isinstance(buildings, list)
        assert len(buildings) == 1

    def test_roads_returns_list(self) -> None:
        roads = GmlReader(ASAM_JUNCTION).finish().roads()
        assert isinstance(roads, list)
        assert len(roads) == 1

    def test_empty_roads_for_building_file(self) -> None:
        assert GmlReader(FZK_LOD2).finish().roads() == []

    def test_empty_buildings_for_road_file(self) -> None:
        assert GmlReader(ASAM_JUNCTION).finish().buildings() == []

    def test_refresh_bounded_by_recursive(self) -> None:
        model = GmlReader(FZK_LOD2).finish()
        model.refresh_bounded_by_recursive()  # must not raise
