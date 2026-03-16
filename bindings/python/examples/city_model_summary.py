"""city_model_summary.py — Print a hierarchical summary of a CityGML file.

Usage:
    python examples/city_model_summary.py <path/to/file.gml>

Example:
    python examples/city_model_summary.py ../../../../tests/fixtures/FZK-Haus-LoD2-KIT-IAI-KHH-B36-V1__v3.gml
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from ecitygml import (
    AuxiliaryTrafficSpace,
    Building,
    CityFurniture,
    GmlReader,
    Intersection,
    ReliefFeature,
    Road,
    Section,
    SolitaryVegetationObject,
    TinRelief,
    TrafficSpace,
)


def _fmt_env(obj: object) -> str:
    env = getattr(obj, "bounded_by", None)
    if env is None:
        return "(no envelope)"
    lc = env.lower_corner
    uc = env.upper_corner
    return (
        f"[{lc[0]:.1f}, {lc[1]:.1f}, {lc[2]:.1f}] → "
        f"[{uc[0]:.1f}, {uc[1]:.1f}, {uc[2]:.1f}]  "
        f"({env.size_x:.1f} × {env.size_y:.1f} × {env.size_z:.1f} m)"
    )


def _summarise_building(b: Building, indent: str = "  ") -> None:
    lods = []
    if b.lod1_solid:
        lods.append(f"LoD1 solid ({len(b.lod1_solid.polygons())} polygons)")
    if b.lod2_solid:
        lods.append(f"LoD2 solid ({len(b.lod2_solid.polygons())} polygons)")
    if b.lod3_solid:
        lods.append(f"LoD3 solid ({len(b.lod3_solid.polygons())} polygons)")
    for attr, label in [
        ("lod0_multi_surface", "LoD0 surface"),
        ("lod2_multi_surface", "LoD2 surface"),
        ("lod3_multi_surface", "LoD3 surface"),
    ]:
        ms = getattr(b, attr)
        if ms is not None:
            lods.append(f"{label} ({len(ms.polygons())} polygons)")

    storeys = ""
    if b.storeys_above_ground is not None:
        storeys = f"  storeys above={b.storeys_above_ground}"
    if b.storeys_below_ground is not None:
        storeys += f" below={b.storeys_below_ground}"

    print(f"{indent}id: {b.id}")
    print(f"{indent}envelope: {_fmt_env(b)}{storeys}")
    if lods:
        print(f"{indent}geometry: {', '.join(lods)}")

    surfaces = [
        ("WallSurface", b.wall_surface),
        ("RoofSurface", b.roof_surface),
        ("GroundSurface", b.ground_surface),
    ]
    for label, items in surfaces:
        if items:
            doors = sum(len(getattr(s, "door_surface", [])) for s in items)
            windows = sum(len(getattr(s, "window_surface", [])) for s in items)
            extra = ""
            if doors:
                extra += f", {doors} door(s)"
            if windows:
                extra += f", {windows} window(s)"
            print(f"{indent}{label}: {len(items)}{extra}")


def _summarise_traffic_space(ts: TrafficSpace | AuxiliaryTrafficSpace, indent: str) -> None:
    geom_parts = []
    if ts.lod2_multi_curve is not None:
        curves = ts.lod2_multi_curve.curves()
        pts = sum(len(c) for c in curves)
        geom_parts.append(f"LoD2 curve ({len(curves)} curve(s), {pts} pts)")
    if ts.lod2_multi_surface is not None:
        geom_parts.append(f"LoD2 surface ({len(ts.lod2_multi_surface.polygons())} polygons)")

    areas = (
        ts.traffic_area
        if hasattr(ts, "traffic_area")
        else ts.auxiliary_traffic_area
    )
    area_label = "traffic_area" if hasattr(ts, "traffic_area") else "auxiliary_traffic_area"

    print(
        f"{indent}{ts.id[:40]}  "
        f"{'  '.join(geom_parts) or '(no geometry)'}  "
        f"| {area_label}: {len(areas)}"
    )


def _summarise_section(sec: Section | Intersection, indent: str = "      ") -> None:
    tag = "Section" if isinstance(sec, Section) else "Intersection"
    print(f"    [{tag}] {sec.id}")
    print(f"    {indent}envelope: {_fmt_env(sec)}")

    if sec.traffic_space:
        print(f"    {indent}traffic_space ({len(sec.traffic_space)}):")
        for ts in sec.traffic_space:
            _summarise_traffic_space(ts, indent + "  ")

    aux = sec.auxiliary_traffic_space if isinstance(sec, Section) else []
    if aux:
        print(f"    {indent}auxiliary_traffic_space ({len(aux)}):")
        for ats in aux:
            _summarise_traffic_space(ats, indent + "  ")


def _summarise_road(road: Road, indent: str = "  ") -> None:
    print(f"{indent}id: {road.id}")
    print(f"{indent}envelope: {_fmt_env(road)}")
    print(f"{indent}sections ({len(road.section)}), intersections ({len(road.intersection)})")
    for sec in road.section:
        _summarise_section(sec, indent="  ")
    for inter in road.intersection:
        _summarise_section(inter, indent="  ")


def main() -> None:
    parser = argparse.ArgumentParser(description="Print a summary of a CityGML file.")
    parser.add_argument("path", help="Path to a .gml file")
    args = parser.parse_args()

    gml_path = Path(args.path)
    if not gml_path.exists():
        print(f"error: file not found: {gml_path}", file=sys.stderr)
        sys.exit(1)

    print(f"Reading {gml_path.name} …")
    model = GmlReader(str(gml_path)).finish()
    print(f"Total city objects: {model.city_objects_len()}\n")

    # ── Buildings ─────────────────────────────────────────────────────────────
    buildings = model.buildings()
    if buildings:
        print(f"{'─' * 60}")
        print(f"Buildings ({len(buildings)})")
        print(f"{'─' * 60}")
        for b in buildings:
            _summarise_building(b)
            print()

    # ── Roads ─────────────────────────────────────────────────────────────────
    roads = model.roads()
    if roads:
        print(f"{'─' * 60}")
        print(f"Roads ({len(roads)})")
        print(f"{'─' * 60}")
        for road in roads:
            _summarise_road(road)
            print()

    # ── Relief ────────────────────────────────────────────────────────────────
    relief_features = model.relief_features()
    if relief_features:
        print(f"{'─' * 60}")
        print(f"ReliefFeatures ({len(relief_features)})")
        print(f"{'─' * 60}")
        for rf in relief_features:
            print(f"  id: {rf.id}  components: {len(rf.relief_component)}")

    tin_reliefs = model.tin_reliefs()
    if tin_reliefs:
        print(f"{'─' * 60}")
        print(f"TinReliefs ({len(tin_reliefs)})")
        print(f"{'─' * 60}")
        for tr in tin_reliefs:
            print(f"  id: {tr.id}  triangles: {len(tr.tin.triangles)}")

    # ── Vegetation & furniture ────────────────────────────────────────────────
    veg = model.solitary_vegetation_objects()
    if veg:
        print(f"{'─' * 60}")
        print(f"SolitaryVegetationObjects ({len(veg)})")
        print(f"{'─' * 60}")
        for v in veg:
            print(f"  id: {v.id}  envelope: {_fmt_env(v)}")

    furniture = model.city_furniture_objects()
    if furniture:
        print(f"{'─' * 60}")
        print(f"CityFurniture ({len(furniture)})")
        print(f"{'─' * 60}")
        for f in furniture:
            print(f"  id: {f.id}  envelope: {_fmt_env(f)}")


if __name__ == "__main__":
    main()
