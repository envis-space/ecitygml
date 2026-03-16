# Python bindings for ecitygml

Python bindings for the ecitygml CityGML 3.0 library. Built with [PyO3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/), targeting Python 3.8+ via the stable ABI.

## Quickstart

```python
from ecitygml import GmlReader, CityObjectClass

model = GmlReader("path/to/city.gml").finish()

# Typed accessors — no casting required
for building in model.buildings():
    print(building.id, building.city_object_class)
    if building.lod2_solid:
        polys = building.lod2_solid.polygons()
        pts = polys[0].exterior.points
        print(f"  first point: x={pts[0].x:.2f} y={pts[0].y:.2f} z={pts[0].z:.2f}")
    for wall in building.wall_surface:
        ms = wall.lod2_multi_surface
        if ms:
            ts = ms.triangulate()
            print(f"  wall: {len(ts.triangles)} triangles")

for road in model.roads():
    for section in road.section:
        for ts in section.traffic_space:
            mc = ts.lod2_multi_curve
            if mc:
                print(f"  curve points: {len(mc.curves()[0])}")
```

## Installation

The native extension must be compiled from source. Install the build prerequisites:

- [Rust toolchain](https://rustup.rs/) (1.83+)
- [maturin](https://www.maturin.rs/) (`pip install maturin` or `uv tool install maturin`)
- Access to the `envis-main-cargo` Cargo registry (for `egml` and `ecitygml` crates)

```bash
# From bindings/python/
maturin develop
```

This compiles the Rust extension and installs `ecitygml` into the active Python environment. For a release build:

```bash
maturin develop --release
```

## Examples

Ready-to-run scripts are in the `examples/` directory:

| Script | What it shows |
|--------|---------------|
| `city_model_summary.py` | Hierarchical summary of every city object in a GML file — buildings with thematic surfaces and storeys, roads with their section/intersection tree, relief, vegetation, and furniture |
| `export_obj.py` | Export all building surfaces (triangulated) to a Wavefront OBJ file, grouped by surface type (WallSurface, RoofSurface, …) with door/window sub-surfaces |
| `road_network.py` | Walk the full transportation hierarchy — Road → Section/Intersection → TrafficSpace/AuxiliaryTrafficSpace → TrafficArea/AuxiliaryTrafficArea — printing centre-line curve lengths and surface polygon counts |

```bash
# From bindings/python/

python examples/city_model_summary.py ../../../../tests/fixtures/FZK-Haus-LoD2-KIT-IAI-KHH-B36-V1__v3.gml

python examples/export_obj.py ../../../../tests/fixtures/FZK-Haus-LoD3-KIT-IAI-KHH-B36-V1__v3.gml
# → FZK-Haus-LoD3-KIT-IAI-KHH-B36-V1__v3.obj

python examples/road_network.py ../../../../tests/fixtures/ASAM-Ex_Bidirectional_Junction.gml
```

## Running Tests

```bash
# From bindings/python/
pytest tests/
```

See [`tests/README.md`](tests/README.md) for details on fixtures and what each test file covers.

## Project Structure

```
bindings/python/
├── Cargo.toml              # Rust package — ecitygml-python
├── pyproject.toml          # Python package — ecitygml (maturin build backend)
├── src/
│   ├── lib.rs              # Module root: registers all types, stub_info gatherer
│   ├── geometry.rs         # DirectPosition, Envelope, LinearRing, Polygon,
│   │                       #   Triangle, TriangulatedSurface, Solid,
│   │                       #   MultiSurface, MultiCurve
│   ├── enums.rs            # LevelOfDetail, CityObjectClass
│   ├── model.rs            # GmlReader, CityModel
│   ├── city_objects.rs     # Building, WallSurface, RoofSurface, GroundSurface,
│   │                       #   DoorSurface, WindowSurface, Road, Section,
│   │                       #   Intersection, TrafficSpace, TrafficArea,
│   │                       #   AuxiliaryTrafficSpace, AuxiliaryTrafficArea,
│   │                       #   SolitaryVegetationObject, CityFurniture,
│   │                       #   ReliefFeature, TinRelief
│   └── bin/
│       └── stub_gen.rs     # Binary: generates _ecitygml.pyi type stubs
├── python/
│   └── ecitygml/
│       ├── __init__.py     # Public API: re-exports all types from ._ecitygml
│       ├── _ecitygml.pyi   # Auto-generated type stubs (checked in)
│       └── py.typed        # PEP 561 marker (enables mypy/pyright support)
└── tests/
    ├── README.md
    ├── conftest.py
    ├── test_reader.py
    ├── test_geometry.py
    ├── test_building.py
    ├── test_transportation.py
    └── test_enums.py
```

### Module layout

The native extension is named `ecitygml._ecitygml` (private). The public `ecitygml` namespace is provided by `python/ecitygml/__init__.py`, which re-exports every type from `._ecitygml`. Users always import from `ecitygml`, never from `ecitygml._ecitygml` directly.

## API Reference

### Reading a file

```python
GmlReader(path: str) -> GmlReader
    .with_rebuild_object_bounds(rebuild: bool) -> GmlReader
    .finish() -> CityModel                  # raises OSError on failure
```

### CityModel

```python
model.city_objects_len() -> int
model.buildings()                   -> list[Building]
model.roads()                       -> list[Road]
model.solitary_vegetation_objects() -> list[SolitaryVegetationObject]
model.city_furniture_objects()      -> list[CityFurniture]
model.relief_features()             -> list[ReliefFeature]
model.tin_reliefs()                 -> list[TinRelief]
model.refresh_bounded_by_recursive()
```

### City objects

All city objects expose:

| Property | Type |
|----------|------|
| `id` | `str` |
| `bounded_by` | `Envelope \| None` |
| `city_object_class` | `CityObjectClass` |

Building-specific:

| Property | Type |
|----------|------|
| `storeys_above_ground` | `int \| None` |
| `storeys_below_ground` | `int \| None` |
| `lod1_solid` / `lod2_solid` / `lod3_solid` | `Solid \| None` |
| `lod0_multi_surface` / `lod2_multi_surface` / `lod3_multi_surface` | `MultiSurface \| None` |
| `wall_surface` | `list[WallSurface]` |
| `roof_surface` | `list[RoofSurface]` |
| `ground_surface` | `list[GroundSurface]` |

Thematic surfaces (`WallSurface`, `RoofSurface`, `GroundSurface`, `DoorSurface`, `WindowSurface`):

| Property | Type |
|----------|------|
| `lod0_multi_surface` / `lod2_multi_surface` / `lod3_multi_surface` | `MultiSurface \| None` |
| `door_surface` | `list[DoorSurface]` — WallSurface only |
| `window_surface` | `list[WindowSurface]` — WallSurface only |

Transportation:

| Type | Key properties |
|------|---------------|
| `Road` | `section: list[Section]`, `intersection: list[Intersection]` |
| `Section` / `Intersection` | `traffic_space`, `auxiliary_traffic_space`, `lod0_multi_curve`, `lod0_multi_surface`, `lod1_solid` |
| `TrafficSpace` | `traffic_area`, `lod2_multi_surface`, `lod2_multi_curve` |
| `AuxiliaryTrafficSpace` | `auxiliary_traffic_area`, `lod2_multi_surface`, `lod2_multi_curve` |
| `TrafficArea` / `AuxiliaryTrafficArea` | `lod2_multi_surface` |

Relief:

| Type | Key properties |
|------|---------------|
| `ReliefFeature` | `relief_component: list[TinRelief]` |
| `TinRelief` | `tin: TriangulatedSurface` |

### Geometry

```python
DirectPosition   .x, .y, .z, .coords  -> tuple[float, float, float]
Envelope         .lower_corner, .upper_corner  -> tuple[float, float, float]
                 .size_x, .size_y, .size_z, .volume  -> float
                 .contains(pos: DirectPosition) -> bool
LinearRing       .points  -> list[DirectPosition]
Polygon          .exterior  -> LinearRing | None
                 .interior  -> list[LinearRing]
                 .compute_envelope() -> Envelope | None
Triangle         .a, .b, .c  -> DirectPosition
TriangulatedSurface  .triangles  -> list[Triangle]
                     .compute_envelope() -> Envelope | None
Solid            .polygons() -> list[Polygon]
                 .triangulate() -> TriangulatedSurface
                 .compute_envelope() -> Envelope | None
MultiSurface     .polygons() -> list[Polygon]
                 .triangulate() -> TriangulatedSurface
                 .compute_envelope() -> Envelope | None
MultiCurve       .curves() -> list[list[DirectPosition]]
                 .compute_envelope() -> Envelope | None
```

### Enums

```python
LevelOfDetail.Zero / One / Two / Three   (.value -> int 0–3)
CityObjectClass.Building / Road / Section / Intersection /
    TrafficSpace / TrafficArea / AuxiliaryTrafficSpace / AuxiliaryTrafficArea /
    WallSurface / RoofSurface / GroundSurface / DoorSurface / WindowSurface /
    TinRelief / ReliefFeature / SolitaryVegetationObject / CityFurniture
```

## Type Stubs

`python/ecitygml/_ecitygml.pyi` is auto-generated by the `stub_gen` binary using [pyo3-stub-gen](https://github.com/Jij-Inc/pyo3-stub-gen). It is checked into the repository so that IDEs and type checkers work without building the extension.

To regenerate after changing the Rust types:

```bash
# macOS — point to the Python dynamic library so the binary can load it
DYLD_LIBRARY_PATH=$(python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))") \
    cargo run --bin stub_gen

# Linux
LD_LIBRARY_PATH=$(python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))") \
    cargo run --bin stub_gen
```

The generated file is written to `python/ecitygml/_ecitygml.pyi`. Verify it is syntactically valid:

```bash
python -c "import ast; ast.parse(open('python/ecitygml/_ecitygml.pyi').read()); print('OK')"
```

## Implementation Notes

### Crate types

`Cargo.toml` declares `crate-type = ["cdylib", "rlib"]`. The `cdylib` produces the `.so`/`.pyd` Python extension; the `rlib` is required so the `stub_gen` binary can link against the library's type-inventory data registered by `pyo3-stub-gen`.

### pyo3-stub-gen version

`pyo3-stub-gen 0.7.0` is used, not the current release. Later versions (0.8+) reference `PyEncodingWarning`, which is only available from Python 3.10 and is absent from the `abi3-py38` stable ABI. Upgrading to a newer version will require either dropping the 3.8 target or waiting for an upstream fix.

### Enum macros

In pyo3-stub-gen 0.7.0, PyO3 enums must be annotated with `#[gen_stub_pyclass_enum]`, not `#[gen_stub_pyclass]`. Adding `#[gen_stub_pymethods]` to enum `impl` blocks will panic at stub generation time.

### Ownership model

All Python wrapper types own their data (cloned from Rust on construction). PyO3 cannot hold Rust borrows across the FFI boundary, so every accessor that returns a sub-object clones it. This keeps the API simple at a small memory cost for large models.
