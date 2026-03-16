"""export_obj.py — Export building geometry from a CityGML file to Wavefront OBJ.

Each surface type (WallSurface, RoofSurface, GroundSurface, DoorSurface,
WindowSurface) becomes a named group in the OBJ file. Geometry is triangulated
before export so the file is ready for any 3D tool.

Usage:
    python examples/export_obj.py <input.gml> [output.obj]

    If output is omitted, the OBJ is written next to the input file with
    the same stem.

Example:
    python examples/export_obj.py ../../../../tests/fixtures/FZK-Haus-LoD3-KIT-IAI-KHH-B36-V1__v3.gml
    # → FZK-Haus-LoD3-KIT-IAI-KHH-B36-V1__v3.obj
"""

from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass, field
from pathlib import Path

from ecitygml import (
    Building,
    DirectPosition,
    DoorSurface,
    GmlReader,
    GroundSurface,
    MultiSurface,
    RoofSurface,
    Triangle,
    WallSurface,
    WindowSurface,
)


@dataclass
class ObjBuilder:
    """Accumulates vertices and triangle faces for OBJ export."""

    vertices: list[tuple[float, float, float]] = field(default_factory=list)
    # group name → list of (v0_idx, v1_idx, v2_idx), 1-based
    groups: dict[str, list[tuple[int, int, int]]] = field(default_factory=dict)

    def _add_vertex(self, pos: DirectPosition) -> int:
        """Append a vertex and return its 1-based OBJ index."""
        self.vertices.append((pos.x, pos.y, pos.z))
        return len(self.vertices)  # 1-based

    def add_triangle(self, group: str, tri: Triangle) -> None:
        ia = self._add_vertex(tri.a)
        ib = self._add_vertex(tri.b)
        ic = self._add_vertex(tri.c)
        self.groups.setdefault(group, []).append((ia, ib, ic))

    def add_multi_surface(self, group: str, ms: MultiSurface) -> None:
        ts = ms.triangulate()
        for tri in ts.triangles:
            self.add_triangle(group, tri)

    def triangle_count(self) -> int:
        return sum(len(faces) for faces in self.groups.values())

    def write(self, path: Path) -> None:
        with path.open("w", encoding="utf-8") as fh:
            fh.write(f"# Exported by ecitygml export_obj.py\n")
            fh.write(f"# {len(self.vertices)} vertices, {self.triangle_count()} triangles\n\n")

            for x, y, z in self.vertices:
                fh.write(f"v {x:.6f} {y:.6f} {z:.6f}\n")

            for group, faces in self.groups.items():
                fh.write(f"\ng {group}\n")
                for ia, ib, ic in faces:
                    fh.write(f"f {ia} {ib} {ic}\n")


def _pick_multi_surface(surface: object) -> MultiSurface | None:
    """Return the highest available LoD multi-surface from a thematic surface."""
    for attr in ("lod3_multi_surface", "lod2_multi_surface", "lod0_multi_surface"):
        ms = getattr(surface, attr, None)
        if ms is not None:
            return ms
    return None


def export_building(building: Building, obj: ObjBuilder) -> None:
    bid = building.id

    # Prefer thematic surfaces (LoD2+); fall back to solid.
    has_thematic = (
        building.wall_surface or building.roof_surface or building.ground_surface
    )

    if has_thematic:
        for ws in building.wall_surface:
            ms = _pick_multi_surface(ws)
            if ms:
                obj.add_multi_surface(f"{bid}_WallSurface_{ws.id}", ms)
            for ds in ws.door_surface:
                dms = _pick_multi_surface(ds)
                if dms:
                    obj.add_multi_surface(f"{bid}_DoorSurface_{ds.id}", dms)
            for win in ws.window_surface:
                wms = _pick_multi_surface(win)
                if wms:
                    obj.add_multi_surface(f"{bid}_WindowSurface_{win.id}", wms)
        for rs in building.roof_surface:
            ms = _pick_multi_surface(rs)
            if ms:
                obj.add_multi_surface(f"{bid}_RoofSurface_{rs.id}", ms)
        for gs in building.ground_surface:
            ms = _pick_multi_surface(gs)
            if ms:
                obj.add_multi_surface(f"{bid}_GroundSurface_{gs.id}", ms)

    else:
        # Fall back to volumetric geometry.
        for attr, label in [
            ("lod3_solid", "LoD3"),
            ("lod2_solid", "LoD2"),
            ("lod1_solid", "LoD1"),
        ]:
            solid = getattr(building, attr, None)
            if solid is not None:
                ts = solid.triangulate()
                for i, tri in enumerate(ts.triangles):
                    obj.add_triangle(f"{bid}_{label}_tri{i}", tri)
                break


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Export building geometry from a CityGML file to Wavefront OBJ."
    )
    parser.add_argument("input", help="Path to a .gml file")
    parser.add_argument("output", nargs="?", help="Output .obj path (default: next to input)")
    args = parser.parse_args()

    gml_path = Path(args.input)
    if not gml_path.exists():
        print(f"error: file not found: {gml_path}", file=sys.stderr)
        sys.exit(1)

    obj_path = Path(args.output) if args.output else gml_path.with_suffix(".obj")

    print(f"Reading {gml_path.name} …")
    model = GmlReader(str(gml_path)).finish()

    buildings = model.buildings()
    if not buildings:
        print("No buildings found.")
        sys.exit(0)

    print(f"Exporting {len(buildings)} building(s) …")
    obj = ObjBuilder()
    for building in buildings:
        export_building(building, obj)

    obj.write(obj_path)
    print(
        f"Wrote {obj_path}  "
        f"({len(obj.vertices)} vertices, {obj.triangle_count()} triangles, "
        f"{len(obj.groups)} groups)"
    )


if __name__ == "__main__":
    main()
