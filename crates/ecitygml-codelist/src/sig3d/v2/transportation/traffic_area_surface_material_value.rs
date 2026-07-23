use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `TrafficArea_surfaceMaterial.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/transportation/2.0/TrafficArea_surfaceMaterial.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dTrafficAreaSurfaceMaterialValue {
    Asphalt,
    Concrete,
    Pavement,
    Cobblestone,
    Gravel,
    RailWithBed,
    RailWithoutBed,
    Soil,
    Sand,
    Grass,
    Wood,
    Steel,
    Marble,
    Unknown,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dTrafficAreaSurfaceMaterialValue {
    const CODE_SPACE: &'static str = "http://www.sig3d.org/codelists/citygml/2.0/transportation/2.0/TrafficArea_surfaceMaterial.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1" => Self::Asphalt,
            "2" => Self::Concrete,
            "3" => Self::Pavement,
            "4" => Self::Cobblestone,
            "5" => Self::Gravel,
            "6" => Self::RailWithBed,
            "7" => Self::RailWithoutBed,
            "8" => Self::Soil,
            "9" => Self::Sand,
            "10" => Self::Grass,
            "11" => Self::Wood,
            "12" => Self::Steel,
            "13" => Self::Marble,
            "99990" => Self::Unknown,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Asphalt => "1",
            Self::Concrete => "2",
            Self::Pavement => "3",
            Self::Cobblestone => "4",
            Self::Gravel => "5",
            Self::RailWithBed => "6",
            Self::RailWithoutBed => "7",
            Self::Soil => "8",
            Self::Sand => "9",
            Self::Grass => "10",
            Self::Wood => "11",
            Self::Steel => "12",
            Self::Marble => "13",
            Self::Unknown => "99990",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::transportation::values::SurfaceMaterialValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1", Sig3dTrafficAreaSurfaceMaterialValue::Asphalt),
            ("99990", Sig3dTrafficAreaSurfaceMaterialValue::Unknown),
        ] {
            assert_eq!(
                Sig3dTrafficAreaSurfaceMaterialValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dTrafficAreaSurfaceMaterialValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dTrafficAreaSurfaceMaterialValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = SurfaceMaterialValue::from(Code::new("1"));
        assert_eq!(
            wrapped.interpret::<Sig3dTrafficAreaSurfaceMaterialValue>(),
            Some(Sig3dTrafficAreaSurfaceMaterialValue::Asphalt)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = SurfaceMaterialValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1",
        ));
        assert_eq!(
            wrapped.interpret::<Sig3dTrafficAreaSurfaceMaterialValue>(),
            None
        );
    }
}
