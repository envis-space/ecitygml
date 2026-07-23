use crate::model::common::arena::InternalKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ArenaProperties {
    pub key: Option<InternalKey>,
}
