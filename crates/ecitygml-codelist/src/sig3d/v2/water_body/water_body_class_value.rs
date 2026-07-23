use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `WaterBody_class.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/waterbody/2.0/WaterBody_class.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dWaterBodyClassValue {
    Sea,
    TidalWaterbody,
    Watercourse,
    RiverStream,
    Ditch,
    SpringWaterHole,
    LakePont,
    Bayou,
    BodyOfStandingWater,
    Waterfall,
    Rapids,
    Swamp,
    SinkholeKarst,
    EphemeralWatercourse,
    FloodedLand,
    ArtificialWaterbody,
    Aqueduct,
    Canal,
    PortBasin,
    Reservior,
    ExcavationPont,
    Moat,
    Pool,
    Fountain,
    Well,
    Cistern,
    FishLadder,
    Unknown,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dWaterBodyClassValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/waterbody/2.0/WaterBody_class.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::Sea,
            "1010" => Self::TidalWaterbody,
            "1020" => Self::Watercourse,
            "1030" => Self::RiverStream,
            "1040" => Self::Ditch,
            "1050" => Self::SpringWaterHole,
            "1060" => Self::LakePont,
            "1070" => Self::Bayou,
            "1080" => Self::BodyOfStandingWater,
            "1090" => Self::Waterfall,
            "1100" => Self::Rapids,
            "1110" => Self::Swamp,
            "1120" => Self::SinkholeKarst,
            "1130" => Self::EphemeralWatercourse,
            "1140" => Self::FloodedLand,
            "1150" => Self::ArtificialWaterbody,
            "1160" => Self::Aqueduct,
            "1170" => Self::Canal,
            "1180" => Self::PortBasin,
            "1190" => Self::Reservior,
            "1200" => Self::ExcavationPont,
            "1210" => Self::Moat,
            "1220" => Self::Pool,
            "1230" => Self::Fountain,
            "1240" => Self::Well,
            "1250" => Self::Cistern,
            "1260" => Self::FishLadder,
            "9999" => Self::Unknown,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Sea => "1000",
            Self::TidalWaterbody => "1010",
            Self::Watercourse => "1020",
            Self::RiverStream => "1030",
            Self::Ditch => "1040",
            Self::SpringWaterHole => "1050",
            Self::LakePont => "1060",
            Self::Bayou => "1070",
            Self::BodyOfStandingWater => "1080",
            Self::Waterfall => "1090",
            Self::Rapids => "1100",
            Self::Swamp => "1110",
            Self::SinkholeKarst => "1120",
            Self::EphemeralWatercourse => "1130",
            Self::FloodedLand => "1140",
            Self::ArtificialWaterbody => "1150",
            Self::Aqueduct => "1160",
            Self::Canal => "1170",
            Self::PortBasin => "1180",
            Self::Reservior => "1190",
            Self::ExcavationPont => "1200",
            Self::Moat => "1210",
            Self::Pool => "1220",
            Self::Fountain => "1230",
            Self::Well => "1240",
            Self::Cistern => "1250",
            Self::FishLadder => "1260",
            Self::Unknown => "9999",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::water_body::values::WaterBodyClassValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dWaterBodyClassValue::Sea),
            ("9999", Sig3dWaterBodyClassValue::Unknown),
        ] {
            assert_eq!(Sig3dWaterBodyClassValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dWaterBodyClassValue::from_code_value("999999");
        assert_eq!(value, Sig3dWaterBodyClassValue::Other("999999".to_string()));
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = WaterBodyClassValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dWaterBodyClassValue>(),
            Some(Sig3dWaterBodyClassValue::Sea)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = WaterBodyClassValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(wrapped.interpret::<Sig3dWaterBodyClassValue>(), None);
    }
}
