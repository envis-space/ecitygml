# ecitygml

A Rust library for processing [CityGML](https://www.ogc.org/standards/citygml/) data along with an inspector GUI.

![alt text](https://sensordb.org/assets/splash/ecitygml-splash.png)

Features:

- CityGML 3.0 data model implementations for buildings, transportation, land use, vegetation, water bodies, relief, city furniture, point clouds, and more
- GML reading and writing, with automatic CityGML version detection
- Geometry operations, including triangulation and spatial transforms (see [egml](https://github.com/envis-space/egml))
- Desktop GUI for inspecting and visualizing CityGML datasets
- CLI for statistics and basic transformations
- Experimental Python, C ABI, and C++ bindings for use outside Rust

> Early stage of development. Developed by [Benedikt Schwab](https://www.asg.ed.tum.de/en/gis/our-team/staff/benedikt-schwab/) at the [TUM Chair of Geoinformatics](https://github.com/tum-gis). Contributions welcome.

---

## ecitygml-gui

A GUI for inspecting CitGML 3.0 datasets.

Download pre-built executables for your platform from the [release page](https://github.com/tum-gis/ecitygml/releases).

On macOS, you need to remove the quarantine attribute after downloading:

```sh
xattr -d com.apple.quarantine ./ecitygml-gui
```

From source:

```sh
cargo install ecitygml-gui@0.0.2-alpha.5 # replace with the latest version
```

---

## ecitygml (library)

A Rust library for reading, writing, and processing CityGML 3.0 data in your own application.

### Installation

```toml
[dependencies]
ecitygml = "0.0.2-alpha.5" # replace with the latest version
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
