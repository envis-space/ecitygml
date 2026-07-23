use slotmap::{DefaultKey, Key};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternalKey(u64);

impl From<u64> for InternalKey {
    fn from(key: u64) -> Self {
        Self(key)
    }
}

impl From<InternalKey> for DefaultKey {
    fn from(key: InternalKey) -> Self {
        let key_data = slotmap::KeyData::from_ffi(key.0);
        key_data.into()
    }
}

impl From<DefaultKey> for InternalKey {
    fn from(key: DefaultKey) -> Self {
        let key_data = key.data().as_ffi();
        InternalKey(key_data)
    }
}
