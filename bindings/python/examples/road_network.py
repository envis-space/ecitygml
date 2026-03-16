"""road_network.py — Walk and print the road network hierarchy from a CityGML file.

Demonstrates the full transportation type tree:
    Road → Section / Intersection
        → TrafficSpace / AuxiliaryTrafficSpace
            → TrafficArea / AuxiliaryTrafficArea (geometry)
            → MultiCurve centre-line (geometry)

Usage:
    python examples/road_network.py <path/to/file.gml>

Example:
    python examples/road_network.py ../../../../tests/fixtures/ASAM-Ex_Bidirectional_Junction.gml
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from ecitygml import (
    AuxiliaryTrafficSpace,
    GmlReader,
    Intersection,
    MultiCurve,
    Road,
    Section,
    TrafficSpace,
)


# ── Formatting helpers ────────────────────────────────────────────────────────


def _short_id(id_: str, max_len: int = 36) -> str:
    return id_ if len(id_) <= max_len else id_[:max_len] + "…"


def _envelope_str(obj: object) -> str:
    env = getattr(obj, "bounded_by", None)
    if env is None:
        return ""
    lc, uc = env.lower_corner, env.upper_corner
    return (
        f"  bbox [{lc[0]:.1f}, {lc[1]:.1f}, {lc[2]:.1f}] → "
        f"[{uc[0]:.1f}, {uc[1]:.1f}, {uc[2]:.1f}]"
    )


def _curve_summary(mc: MultiCurve) -> str:
    curves = mc.curves()
    total_pts = sum(len(c) for c in curves)
    env = mc.compute_envelope()
    length = ""
    if env is not None:
        # Rough bounding-box diagonal as a proxy for total curve extent.
        diag = (env.size_x**2 + env.size_y**2 + env.size_z**2) ** 0.5
        length = f"  bbox-diag ≈ {diag:.1f} m"
    return f"{len(curves)} curve(s), {total_pts} pts{length}"


def _surface_summary(obj: object, lod_attr: str) -> str:
    ms = getattr(obj, lod_attr, None)
    if ms is None:
        return "(none)"
    polys = ms.polygons()
    triangles = ms.triangulate().triangles
    return f"{len(polys)} polygon(s), {len(triangles)} triangle(s)"


# ── Per-type printers ─────────────────────────────────────────────────────────


def print_traffic_space(ts: TrafficSpace | AuxiliaryTrafficSpace, indent: str) -> None:
    label = "TrafficSpace" if isinstance(ts, TrafficSpace) else "AuxiliaryTrafficSpace"
    print(f"{indent}[{label}] {_short_id(ts.id)}{_envelope_str(ts)}")

    if ts.lod2_multi_curve is not None:
        print(f"{indent}  centre-line: {_curve_summary(ts.lod2_multi_curve)}")

    if ts.lod2_multi_surface is not None:
        print(f"{indent}  lod2_surface: {_surface_summary(ts, 'lod2_multi_surface')}")

    if isinstance(ts, TrafficSpace):
        areas = ts.traffic_area
        area_label = "TrafficArea"
    else:
        areas = ts.auxiliary_traffic_area
        area_label = "AuxiliaryTrafficArea"

    for area in areas:
        ms_str = _surface_summary(area, "lod2_multi_surface")
        print(f"{indent}  [{area_label}] {_short_id(area.id)}  surface: {ms_str}")


def print_section(sec: Section, indent: str) -> None:
    print(f"{indent}[Section] {_short_id(sec.id)}{_envelope_str(sec)}")

    if sec.lod0_multi_curve is not None:
        print(f"{indent}  lod0_curve: {_curve_summary(sec.lod0_multi_curve)}")
    if sec.lod0_multi_surface is not None:
        print(f"{indent}  lod0_surface: {_surface_summary(sec, 'lod0_multi_surface')}")
    if sec.lod1_solid is not None:
        polys = sec.lod1_solid.polygons()
        tris = sec.lod1_solid.triangulate().triangles
        print(f"{indent}  lod1_solid: {len(polys)} polygons, {len(tris)} triangles")

    for ts in sec.traffic_space:
        print_traffic_space(ts, indent + "  ")

    for ats in sec.auxiliary_traffic_space:
        print_traffic_space(ats, indent + "  ")


def print_intersection(inter: Intersection, indent: str) -> None:
    print(f"{indent}[Intersection] {_short_id(inter.id)}{_envelope_str(inter)}")
    for ts in inter.traffic_space:
        print_traffic_space(ts, indent + "  ")
    for ats in inter.auxiliary_traffic_space:
        print_traffic_space(ats, indent + "  ")


def print_road(road: Road) -> None:
    print(f"[Road] {road.id}{_envelope_str(road)}")
    print(f"  {len(road.section)} section(s), {len(road.intersection)} intersection(s)")
    print()

    for sec in road.section:
        print_section(sec, indent="  ")
        print()

    for inter in road.intersection:
        print_intersection(inter, indent="  ")
        print()


# ── Entry point ───────────────────────────────────────────────────────────────


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Print the road network hierarchy from a CityGML file."
    )
    parser.add_argument("path", help="Path to a .gml file")
    args = parser.parse_args()

    gml_path = Path(args.path)
    if not gml_path.exists():
        print(f"error: file not found: {gml_path}", file=sys.stderr)
        sys.exit(1)

    print(f"Reading {gml_path.name} …\n")
    model = GmlReader(str(gml_path)).finish()

    roads = model.roads()
    if not roads:
        print("No roads found in this file.")
        sys.exit(0)

    print(f"Found {len(roads)} road(s)\n{'=' * 60}\n")
    for road in roads:
        print_road(road)


if __name__ == "__main__":
    main()
