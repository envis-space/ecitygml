use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `CityObjectGroup_function.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/cityobjectgroup/2.0/CityObjectGroup_function.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dCityObjectGroupFunctionValue {
    Lod1storey,
    Lod2storey,
    Lod3storey,
    Lod4storey,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dCityObjectGroupFunctionValue {
    const CODE_SPACE: &'static str = "http://www.sig3d.org/codelists/citygml/2.0/cityobjectgroup/2.0/CityObjectGroup_function.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::Lod1storey,
            "1010" => Self::Lod2storey,
            "1020" => Self::Lod3storey,
            "1030" => Self::Lod4storey,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Lod1storey => "1000",
            Self::Lod2storey => "1010",
            Self::Lod3storey => "1020",
            Self::Lod4storey => "1030",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dCityObjectGroupFunctionValue::Lod1storey),
            ("1030", Sig3dCityObjectGroupFunctionValue::Lod4storey),
        ] {
            assert_eq!(
                Sig3dCityObjectGroupFunctionValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dCityObjectGroupFunctionValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dCityObjectGroupFunctionValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }
}
