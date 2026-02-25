use strum_macros::Display;

#[derive(Debug, Copy, Hash, Eq, Clone, PartialEq, Display)]
pub enum TrafficDirectionValue {
    Forwards,
    Backwards,
    Both,
}
