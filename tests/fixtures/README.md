# Datasets

## FZK Haus

The [FZK Haus datasets](https://www.citygmlwiki.org/index.php?title=FZK_Haus) were upgraded to CityGML 3.0 using [citygml-tools](https://github.com/citygml4j/citygml-tools).

## TUM

The TUM main entrance dataset is originally from the Bavarian State Mapping Agency's [OpenData portal](https://geodaten.bayern.de/opengeodata/OpenDataDetail.html?pn=lod2).
The tile [690_5336.gml](https://download1.bayernwolke.de/a/lod2/citygml/690_5336.gml) was converted to CityGML 3.0 using [citygml-tools](https://github.com/citygml4j/citygml-tools) and then filtered for the main entrance with ID `DEBY_LOD2_4959457`:
```bash
subset -i DEBY_LOD2_4959457 ./690_5336.gml
```

## ASAM

The ASAM examples were converted from the official [OpenDRIVE examples](https://www.asam.net/standards/detail/opendrive/) to CityGML 3.0 using [rtron](https://github.com/tum-gis/rtron).

```bash
opendrive-to-citygml ./ASAM_examples_181/Ex_Bidirectional_Junction ./output --discretization-step-size 5.0 --skip-lane-surface-extrusions
```

```bash
opendrive-to-citygml ./ASAM_examples_181/UC_RoadShape ./output --discretization-step-size 50.0
```
