"""Tests for geometry types: DirectPosition, Envelope, LinearRing, Polygon,
Solid, TriangulatedSurface, MultiSurface, MultiCurve."""

import pytest

from ecitygml import (
    Building,
    DirectPosition,
    Envelope,
    LinearRing,
    MultiSurface,
    Polygon,
    Solid,
    Triangle,
    TriangulatedSurface,
)


class TestDirectPosition:
    def test_x_y_z_values(self, fzk_lod1_solid: Solid) -> None:
        pt = fzk_lod1_solid.polygons()[0].exterior.points[0]
        assert pt.x == pytest.approx(457842.0)
        assert pt.y == pytest.approx(5439083.0)
        assert pt.z == pytest.approx(111.8)

    def test_coords_matches_xyz(self, fzk_lod1_solid: Solid) -> None:
        pt = fzk_lod1_solid.polygons()[0].exterior.points[0]
        assert pt.coords == (pt.x, pt.y, pt.z)

    def test_repr_is_non_empty(self, fzk_lod1_solid: Solid) -> None:
        pt = fzk_lod1_solid.polygons()[0].exterior.points[0]
        assert repr(pt)


class TestEnvelope:
    def test_size_x(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.bounded_by.size_x == pytest.approx(12.0)

    def test_size_y(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.bounded_by.size_y == pytest.approx(10.0)

    def test_size_z(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.bounded_by.size_z == pytest.approx(6.5, abs=0.1)

    def test_lower_corner(self, fzk_lod1_building: Building) -> None:
        lc = fzk_lod1_building.bounded_by.lower_corner
        assert lc == pytest.approx((457842.0, 5439083.0, 111.8))

    def test_upper_corner_x(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.bounded_by.upper_corner[0] == pytest.approx(457854.0)

    def test_volume_positive(self, fzk_lod1_building: Building) -> None:
        assert fzk_lod1_building.bounded_by.volume > 0

    def test_contains_returns_bool(self, fzk_lod1_building: Building) -> None:
        env = fzk_lod1_building.bounded_by
        pt = fzk_lod1_building.lod1_solid.polygons()[0].exterior.points[0]
        assert isinstance(env.contains(pt), bool)

    def test_repr_is_non_empty(self, fzk_lod1_building: Building) -> None:
        assert repr(fzk_lod1_building.bounded_by)


class TestLinearRing:
    def test_points_are_direct_positions(self, fzk_lod1_solid: Solid) -> None:
        ring = fzk_lod1_solid.polygons()[0].exterior
        assert isinstance(ring, LinearRing)
        for pt in ring.points:
            assert isinstance(pt, DirectPosition)

    def test_point_count(self, fzk_lod1_solid: Solid) -> None:
        assert len(fzk_lod1_solid.polygons()[0].exterior.points) == 4


class TestPolygon:
    def test_exterior_is_linear_ring(self, fzk_lod1_solid: Solid) -> None:
        assert isinstance(fzk_lod1_solid.polygons()[0].exterior, LinearRing)

    def test_no_interior_rings(self, fzk_lod1_solid: Solid) -> None:
        assert fzk_lod1_solid.polygons()[0].interior == []

    def test_compute_envelope_returns_envelope(self, fzk_lod1_solid: Solid) -> None:
        assert isinstance(fzk_lod1_solid.polygons()[0].compute_envelope(), Envelope)


class TestSolid:
    def test_polygon_count(self, fzk_lod1_solid: Solid) -> None:
        assert len(fzk_lod1_solid.polygons()) == 6

    def test_polygons_are_polygon_instances(self, fzk_lod1_solid: Solid) -> None:
        for poly in fzk_lod1_solid.polygons():
            assert isinstance(poly, Polygon)

    def test_triangulate_returns_triangulated_surface(self, fzk_lod1_solid: Solid) -> None:
        assert isinstance(fzk_lod1_solid.triangulate(), TriangulatedSurface)

    def test_triangulate_count(self, fzk_lod1_solid: Solid) -> None:
        assert len(fzk_lod1_solid.triangulate().triangles) == 12

    def test_compute_envelope_returns_envelope(self, fzk_lod1_solid: Solid) -> None:
        assert isinstance(fzk_lod1_solid.compute_envelope(), Envelope)


class TestTriangulatedSurface:
    def test_triangles_are_triangle_instances(self, fzk_lod1_solid: Solid) -> None:
        for tri in fzk_lod1_solid.triangulate().triangles:
            assert isinstance(tri, Triangle)

    def test_compute_envelope_returns_envelope(self, fzk_lod1_solid: Solid) -> None:
        assert isinstance(fzk_lod1_solid.triangulate().compute_envelope(), Envelope)

    def test_triangle_vertices_are_direct_positions(self, fzk_lod1_solid: Solid) -> None:
        tri = fzk_lod1_solid.triangulate().triangles[0]
        assert isinstance(tri.a, DirectPosition)
        assert isinstance(tri.b, DirectPosition)
        assert isinstance(tri.c, DirectPosition)


class TestMultiSurface:
    def test_polygons_are_polygon_instances(self, fzk_lod2_building: Building) -> None:
        ms = fzk_lod2_building.wall_surface[0].lod2_multi_surface
        for poly in ms.polygons():
            assert isinstance(poly, Polygon)

    def test_triangulate_returns_triangulated_surface(self, fzk_lod2_building: Building) -> None:
        ms = fzk_lod2_building.wall_surface[0].lod2_multi_surface
        assert isinstance(ms.triangulate(), TriangulatedSurface)

    def test_compute_envelope_returns_envelope(self, fzk_lod2_building: Building) -> None:
        ms = fzk_lod2_building.wall_surface[0].lod2_multi_surface
        assert isinstance(ms.compute_envelope(), Envelope)

    def test_repr_is_non_empty(self, fzk_lod2_building: Building) -> None:
        assert repr(fzk_lod2_building.wall_surface[0].lod2_multi_surface)
