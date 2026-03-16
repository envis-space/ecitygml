use strum_macros::Display;

#[derive(Debug, Copy, Hash, Eq, Clone, PartialEq, Display)]
pub enum RelativeToWater {
    EntirelyAboveWaterSurface,
    SubstantiallyAboveWaterSurface,
    SubstantiallyAboveAndBelowWaterSurface,
    SubstantiallyBelowWaterSurface,
    EntirelyBelowWaterSurface,
    TemporarilyAboveAndBelowWaterSurface,
}
