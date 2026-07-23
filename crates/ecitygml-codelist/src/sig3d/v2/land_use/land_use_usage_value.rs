use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `LandUse_usage.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/landuse/2.0/LandUse_usage.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dLandUseUsageValue {
    Residential,
    IndustryAndBusiness,
    MixedUse,
    SpecialFunctionArea,
    Monument,
    Dump,
    Mining,
    Park,
    Cemetary,
    SportsLeisureAndRecreation,
    OpenPitQuarry,
    Road,
    Railway,
    Airfield,
    Shipping,
    Track,
    Square,
    Grassland,
    Agriculture,
    Forest,
    Grove,
    Heath,
    Moor,
    Marsh,
    UntilledLand,
    River,
    StandingWaterbody,
    Harbour,
    Sea,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dLandUseUsageValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/landuse/2.0/LandUse_usage.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1010" => Self::Residential,
            "1020" => Self::IndustryAndBusiness,
            "1030" => Self::MixedUse,
            "1040" => Self::SpecialFunctionArea,
            "1050" => Self::Monument,
            "1060" => Self::Dump,
            "1070" => Self::Mining,
            "1110" => Self::Park,
            "1120" => Self::Cemetary,
            "1130" => Self::SportsLeisureAndRecreation,
            "1140" => Self::OpenPitQuarry,
            "2010" => Self::Road,
            "2020" => Self::Railway,
            "2030" => Self::Airfield,
            "2040" => Self::Shipping,
            "2050" => Self::Track,
            "2060" => Self::Square,
            "3010" => Self::Grassland,
            "3020" => Self::Agriculture,
            "3030" => Self::Forest,
            "3040" => Self::Grove,
            "3050" => Self::Heath,
            "3060" => Self::Moor,
            "3070" => Self::Marsh,
            "3080" => Self::UntilledLand,
            "4010" => Self::River,
            "4020" => Self::StandingWaterbody,
            "4030" => Self::Harbour,
            "4040" => Self::Sea,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Residential => "1010",
            Self::IndustryAndBusiness => "1020",
            Self::MixedUse => "1030",
            Self::SpecialFunctionArea => "1040",
            Self::Monument => "1050",
            Self::Dump => "1060",
            Self::Mining => "1070",
            Self::Park => "1110",
            Self::Cemetary => "1120",
            Self::SportsLeisureAndRecreation => "1130",
            Self::OpenPitQuarry => "1140",
            Self::Road => "2010",
            Self::Railway => "2020",
            Self::Airfield => "2030",
            Self::Shipping => "2040",
            Self::Track => "2050",
            Self::Square => "2060",
            Self::Grassland => "3010",
            Self::Agriculture => "3020",
            Self::Forest => "3030",
            Self::Grove => "3040",
            Self::Heath => "3050",
            Self::Moor => "3060",
            Self::Marsh => "3070",
            Self::UntilledLand => "3080",
            Self::River => "4010",
            Self::StandingWaterbody => "4020",
            Self::Harbour => "4030",
            Self::Sea => "4040",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::land_use::values::LandUseUsageValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1010", Sig3dLandUseUsageValue::Residential),
            ("4040", Sig3dLandUseUsageValue::Sea),
        ] {
            assert_eq!(Sig3dLandUseUsageValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dLandUseUsageValue::from_code_value("999999");
        assert_eq!(value, Sig3dLandUseUsageValue::Other("999999".to_string()));
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = LandUseUsageValue::from(Code::new("1010"));
        assert_eq!(
            wrapped.interpret::<Sig3dLandUseUsageValue>(),
            Some(Sig3dLandUseUsageValue::Residential)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = LandUseUsageValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1010",
        ));
        assert_eq!(wrapped.interpret::<Sig3dLandUseUsageValue>(), None);
    }
}
