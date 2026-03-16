# Python Binding Tests

Integration tests for the `ecitygml` Python bindings. Each test loads one of
the CityGML 3.0 fixture files from `tests/fixtures/` and asserts on the parsed
object tree.

## Fixtures

| File | Contents |
|------|----------|
| `FZK-Haus-LoD1-KIT-IAI-KHH-B36-V1__v3.gml` | Single building with a LoD1 solid (6 polygons, 12 triangles after triangulation) |
| `FZK-Haus-LoD2-KIT-IAI-KHH-B36-V1__v3.gml` | Same building at LoD2: 4 wall surfaces, 2 roof surfaces, 1 ground surface |
| `FZK-Haus-LoD3-KIT-IAI-KHH-B36-V1__v3.gml` | Same building at LoD3: adds door and window surfaces inside wall surfaces |
| `TUM-Main-Entrance.gml` | Bavarian OpenData LoD2 building (TUM main entrance, ID `DEBY_LOD2_4959457`) |
| `ASAM-Ex_Bidirectional_Junction.gml` | Road with 3 sections, 1 intersection, traffic spaces and traffic areas |
| `ASAM-UC_RoadShape.gml` | Road with 1 section containing auxiliary traffic spaces and areas |

## Test Files

| File | What it covers |
|------|----------------|
| `conftest.py` | Shared `module`-scoped fixtures and fixture file paths |
| `test_reader.py` | `GmlReader` and `CityModel` — parsing, error handling, typed accessors |
| `test_geometry.py` | `DirectPosition`, `Envelope`, `LinearRing`, `Polygon`, `Solid`, `TriangulatedSurface`, `MultiSurface` |
| `test_building.py` | `Building` at LoD1/LoD2/LoD3 and TUM, plus `WallSurface`, `RoofSurface`, `GroundSurface`, `DoorSurface`, `WindowSurface` |
| `test_transportation.py` | `Road`, `Section`, `Intersection`, `TrafficSpace`, `TrafficArea`, `AuxiliaryTrafficSpace`, `AuxiliaryTrafficArea`, `MultiCurve` |
| `test_enums.py` | `CityObjectClass` and `LevelOfDetail` |

## Running

```bash
# From bindings/python/
pytest tests/
```

Requires the native extension to be built first:

```bash
maturin develop
pytest tests/
```
