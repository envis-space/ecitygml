"""Shared fixtures and fixture file paths for all ecitygml test modules."""

from pathlib import Path

import pytest

from ecitygml import Building, GmlReader, Road, Section, Solid

FIXTURES = Path(__file__).parent.parent.parent.parent / "tests" / "fixtures"

FZK_LOD1 = str(FIXTURES / "FZK-Haus-LoD1-KIT-IAI-KHH-B36-V1__v3.gml")
FZK_LOD2 = str(FIXTURES / "FZK-Haus-LoD2-KIT-IAI-KHH-B36-V1__v3.gml")
FZK_LOD3 = str(FIXTURES / "FZK-Haus-LoD3-KIT-IAI-KHH-B36-V1__v3.gml")
TUM = str(FIXTURES / "TUM-Main-Entrance.gml")
ASAM_JUNCTION = str(FIXTURES / "ASAM-Ex_Bidirectional_Junction.gml")
ASAM_ROAD = str(FIXTURES / "ASAM-UC_RoadShape.gml")


@pytest.fixture(scope="module")
def fzk_lod1_building() -> Building:
    return GmlReader(FZK_LOD1).finish().buildings()[0]


@pytest.fixture(scope="module")
def fzk_lod2_building() -> Building:
    return GmlReader(FZK_LOD2).finish().buildings()[0]


@pytest.fixture(scope="module")
def fzk_lod3_building() -> Building:
    return GmlReader(FZK_LOD3).finish().buildings()[0]


@pytest.fixture(scope="module")
def tum_building() -> Building:
    return GmlReader(TUM).finish().buildings()[0]


@pytest.fixture(scope="module")
def asam_junction_road() -> Road:
    return GmlReader(ASAM_JUNCTION).finish().roads()[0]


@pytest.fixture(scope="module")
def asam_road_shape_road() -> Road:
    return GmlReader(ASAM_ROAD).finish().roads()[0]


@pytest.fixture(scope="module")
def asam_section(asam_junction_road: Road) -> Section:
    return asam_junction_road.section[0]


@pytest.fixture(scope="module")
def asam_road_section(asam_road_shape_road: Road) -> Section:
    return asam_road_shape_road.section[0]


@pytest.fixture(scope="module")
def fzk_lod1_solid(fzk_lod1_building: Building) -> Solid:
    solid = fzk_lod1_building.lod1_solid
    assert solid is not None
    return solid
