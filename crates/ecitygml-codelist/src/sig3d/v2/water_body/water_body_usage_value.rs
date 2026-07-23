use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `WaterBody_usage.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/waterbody/2.0/WaterBody_usage.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dWaterBodyUsageValue {
    Sanctuary,
    RecreationSports,
    DrinkingWaterSupply,
    HydroelectricWaterSupply,
    OceanShipping,
    InlandShipping,
    Sewer,
    Port,
    Anchorage,
    PublicUse,
    PrivateUse,
    IndustrialCraftWaterSupply,
    MilitaryUse,
    MiningExcavation,
    IrrigationWaterSupply,
    FishingWater,
    FishFarm,
    ArchaeologicalSite,
    WaterProtectionArea,
    Abandoned,
    Unknown,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dWaterBodyUsageValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/waterbody/2.0/WaterBody_usage.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::Sanctuary,
            "1010" => Self::RecreationSports,
            "1020" => Self::DrinkingWaterSupply,
            "1030" => Self::HydroelectricWaterSupply,
            "1040" => Self::OceanShipping,
            "1050" => Self::InlandShipping,
            "1060" => Self::Sewer,
            "1070" => Self::Port,
            "1080" => Self::Anchorage,
            "1090" => Self::PublicUse,
            "1100" => Self::PrivateUse,
            "1110" => Self::IndustrialCraftWaterSupply,
            "1120" => Self::MilitaryUse,
            "1130" => Self::MiningExcavation,
            "1140" => Self::IrrigationWaterSupply,
            "1150" => Self::FishingWater,
            "1160" => Self::FishFarm,
            "1170" => Self::ArchaeologicalSite,
            "1180" => Self::WaterProtectionArea,
            "1190" => Self::Abandoned,
            "9999" => Self::Unknown,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Sanctuary => "1000",
            Self::RecreationSports => "1010",
            Self::DrinkingWaterSupply => "1020",
            Self::HydroelectricWaterSupply => "1030",
            Self::OceanShipping => "1040",
            Self::InlandShipping => "1050",
            Self::Sewer => "1060",
            Self::Port => "1070",
            Self::Anchorage => "1080",
            Self::PublicUse => "1090",
            Self::PrivateUse => "1100",
            Self::IndustrialCraftWaterSupply => "1110",
            Self::MilitaryUse => "1120",
            Self::MiningExcavation => "1130",
            Self::IrrigationWaterSupply => "1140",
            Self::FishingWater => "1150",
            Self::FishFarm => "1160",
            Self::ArchaeologicalSite => "1170",
            Self::WaterProtectionArea => "1180",
            Self::Abandoned => "1190",
            Self::Unknown => "9999",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::water_body::values::WaterBodyUsageValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dWaterBodyUsageValue::Sanctuary),
            ("9999", Sig3dWaterBodyUsageValue::Unknown),
        ] {
            assert_eq!(Sig3dWaterBodyUsageValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dWaterBodyUsageValue::from_code_value("999999");
        assert_eq!(value, Sig3dWaterBodyUsageValue::Other("999999".to_string()));
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = WaterBodyUsageValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dWaterBodyUsageValue>(),
            Some(Sig3dWaterBodyUsageValue::Sanctuary)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = WaterBodyUsageValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(wrapped.interpret::<Sig3dWaterBodyUsageValue>(), None);
    }
}
