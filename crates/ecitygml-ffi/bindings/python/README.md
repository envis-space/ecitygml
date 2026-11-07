# ecitygml Python bindings

Python bindings for [ecitygml](https://github.com/tum-gis/ecitygml), generated with [Diplomat](https://github.com/rust-diplomat/diplomat) (nanobind) into `src/`. Requires CPython ≥ 3.12.

```sh
pip install --pre ecitygml   # --pre while only pre-release versions are published
```

```python
from ecitygml import GmlReader

city_model = GmlReader.from_path("model.gml").finish()
print(city_model.city_objects_len())
```

See [`examples/`](https://github.com/tum-gis/ecitygml/tree/main/crates/ecitygml-ffi/bindings/python/examples) for more.

```sh
mise run build:bindings:python
mise run test:bindings:python
```

See the [ecitygml-ffi README](https://github.com/tum-gis/ecitygml/tree/main/crates/ecitygml-ffi) for details.
