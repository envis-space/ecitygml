use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `CityFurniture_class.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/cityfurniture/2.0/CityFurniture_class.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dCityFurnitureClassValue {
    Traffic,
    Communication,
    Security,
    Others,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dCityFurnitureClassValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/cityfurniture/2.0/CityFurniture_class.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::Traffic,
            "1010" => Self::Communication,
            "1020" => Self::Security,
            "1030" => Self::Others,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Traffic => "1000",
            Self::Communication => "1010",
            Self::Security => "1020",
            Self::Others => "1030",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::city_furniture::values::CityFurnitureClassValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dCityFurnitureClassValue::Traffic),
            ("1030", Sig3dCityFurnitureClassValue::Others),
        ] {
            assert_eq!(Sig3dCityFurnitureClassValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dCityFurnitureClassValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dCityFurnitureClassValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = CityFurnitureClassValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dCityFurnitureClassValue>(),
            Some(Sig3dCityFurnitureClassValue::Traffic)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = CityFurnitureClassValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(wrapped.interpret::<Sig3dCityFurnitureClassValue>(), None);
    }
}
