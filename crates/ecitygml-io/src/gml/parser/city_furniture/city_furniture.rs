use crate::Error;
use crate::gml::parser::core::parse_abstract_occupied_space;
use ecitygml_core::model::city_furniture::CityFurniture;

pub fn parse_city_furniture(xml_document: &[u8]) -> Result<CityFurniture, Error> {
    let occupied_space = parse_abstract_occupied_space(xml_document)?;
    let city_furniture = CityFurniture::new(occupied_space);

    Ok(city_furniture)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractOccupiedSpace, AsAbstractSpace,
    };
    use egml::model::base::Id;

    #[test]
    fn test_parse_basic_city_furniture() {
        let xml_document = b"<frn:CityFurniture gml:id=\"UUID_379c7ee8-b010-33d2-8e55-786e6be6fa46\">
      <gml:name>Vorschriftzeichen</gml:name>
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>identifier_roadObjectName</gen:name>
          <gen:value>Vorschriftzeichen</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <lod2MultiSurface>
        <gml:MultiSurface>
          <gml:surfaceMember>
            <gml:Polygon>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList srsDimension=\"3\">678101.3310583181 5403639.968419715 370.3474251363769 678101.3310583181 5403639.968419715 369.92742513637694 678101.7298154243 5403640.100301367 369.92742513637694 678101.7298154243 5403640.100301367 370.3474251363769 678101.3310583181 5403639.968419715 370.3474251363769</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Polygon>
          </gml:surfaceMember>
        </gml:MultiSurface>
      </lod2MultiSurface>
      <lod1ImplicitRepresentation>
        <ImplicitGeometry>
          <transformationMatrix>-0.3140039352235152 -0.9494216811639318 0.0 0.0 0.9494216811639318 -0.3140039352235152 0.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 0.0 1.0</transformationMatrix>
          <referencePoint>
            <gml:Point>
              <gml:pos srsDimension=\"3\">678101.5304368712 5403640.034360541 369.92742513637694</gml:pos>
            </gml:Point>
          </referencePoint>
        </ImplicitGeometry>
      </lod1ImplicitRepresentation>
    </frn:CityFurniture>";

        let city_furniture = parse_city_furniture(xml_document).expect("should work");

        assert_eq!(
            city_furniture.id(),
            &Id::try_from("UUID_379c7ee8-b010-33d2-8e55-786e6be6fa46").expect("should work")
        );
        assert_eq!(
            city_furniture.name().first().expect("should work"),
            "Vorschriftzeichen"
        );
        assert!(city_furniture.lod1_implicit_representation().is_some());
        assert!(city_furniture.lod2_multi_surface().is_some());
        assert_eq!(city_furniture.generic_attributes().len(), 1);
    }
}
