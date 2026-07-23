use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `LandUse_class.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/landuse/2.0/LandUse_class.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dLandUseClassValue {
    SettlementArea,
    UndevelopedArea,
    Traffic,
    Vegetation,
    Water,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dLandUseClassValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/landuse/2.0/LandUse_class.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::SettlementArea,
            "1100" => Self::UndevelopedArea,
            "2000" => Self::Traffic,
            "3000" => Self::Vegetation,
            "4000" => Self::Water,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::SettlementArea => "1000",
            Self::UndevelopedArea => "1100",
            Self::Traffic => "2000",
            Self::Vegetation => "3000",
            Self::Water => "4000",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::land_use::values::LandUseClassValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dLandUseClassValue::SettlementArea),
            ("4000", Sig3dLandUseClassValue::Water),
        ] {
            assert_eq!(Sig3dLandUseClassValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dLandUseClassValue::from_code_value("999999");
        assert_eq!(value, Sig3dLandUseClassValue::Other("999999".to_string()));
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = LandUseClassValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dLandUseClassValue>(),
            Some(Sig3dLandUseClassValue::SettlementArea)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = LandUseClassValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(wrapped.interpret::<Sig3dLandUseClassValue>(), None);
    }
}
