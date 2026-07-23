use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `AuxiliaryTrafficArea_function.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/transportation/2.0/AuxiliaryTrafficArea_function.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dAuxiliaryTrafficAreaFunctionValue {
    SoftShoulder,
    HardShoulder,
    GreenArea,
    MiddleLane,
    LayBy,
    ParkingBay,
    Ditch,
    Drainage,
    Kerbstone,
    FlowerTub,
    TrafficIsland,
    Bank,
    EmbankmentDike,
    RailroadEmbankment,
    NoiseProtection,
    NoiseProtectionWall,
    NoiseGuardBar,
    Towpath,
    Others,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dAuxiliaryTrafficAreaFunctionValue {
    const CODE_SPACE: &'static str = "http://www.sig3d.org/codelists/citygml/2.0/transportation/2.0/AuxiliaryTrafficArea_function.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::SoftShoulder,
            "1010" => Self::HardShoulder,
            "1020" => Self::GreenArea,
            "1030" => Self::MiddleLane,
            "1040" => Self::LayBy,
            "1100" => Self::ParkingBay,
            "1200" => Self::Ditch,
            "1210" => Self::Drainage,
            "1220" => Self::Kerbstone,
            "1230" => Self::FlowerTub,
            "1300" => Self::TrafficIsland,
            "1400" => Self::Bank,
            "1410" => Self::EmbankmentDike,
            "1420" => Self::RailroadEmbankment,
            "1430" => Self::NoiseProtection,
            "1440" => Self::NoiseProtectionWall,
            "1500" => Self::NoiseGuardBar,
            "1600" => Self::Towpath,
            "1700" => Self::Others,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::SoftShoulder => "1000",
            Self::HardShoulder => "1010",
            Self::GreenArea => "1020",
            Self::MiddleLane => "1030",
            Self::LayBy => "1040",
            Self::ParkingBay => "1100",
            Self::Ditch => "1200",
            Self::Drainage => "1210",
            Self::Kerbstone => "1220",
            Self::FlowerTub => "1230",
            Self::TrafficIsland => "1300",
            Self::Bank => "1400",
            Self::EmbankmentDike => "1410",
            Self::RailroadEmbankment => "1420",
            Self::NoiseProtection => "1430",
            Self::NoiseProtectionWall => "1440",
            Self::NoiseGuardBar => "1500",
            Self::Towpath => "1600",
            Self::Others => "1700",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::transportation::values::AuxiliaryTrafficAreaFunctionValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dAuxiliaryTrafficAreaFunctionValue::SoftShoulder),
            ("1700", Sig3dAuxiliaryTrafficAreaFunctionValue::Others),
        ] {
            assert_eq!(
                Sig3dAuxiliaryTrafficAreaFunctionValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dAuxiliaryTrafficAreaFunctionValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dAuxiliaryTrafficAreaFunctionValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = AuxiliaryTrafficAreaFunctionValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dAuxiliaryTrafficAreaFunctionValue>(),
            Some(Sig3dAuxiliaryTrafficAreaFunctionValue::SoftShoulder)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = AuxiliaryTrafficAreaFunctionValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(
            wrapped.interpret::<Sig3dAuxiliaryTrafficAreaFunctionValue>(),
            None
        );
    }
}
