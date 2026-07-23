use crate::model::core::values::QualifiedVolumeTypeValue;
use egml::model::measures::Volume;

#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedVolume {
    volume: Volume,
    type_of_volume: QualifiedVolumeTypeValue,
}

impl QualifiedVolume {
    pub fn new(volume: Volume, type_of_volume: QualifiedVolumeTypeValue) -> Self {
        Self {
            volume,
            type_of_volume,
        }
    }

    pub fn volume(&self) -> &Volume {
        &self.volume
    }

    pub fn set_volume(&mut self, volume: Volume) {
        self.volume = volume;
    }

    pub fn type_of_volume(&self) -> &QualifiedVolumeTypeValue {
        &self.type_of_volume
    }

    pub fn set_type_of_volume(&mut self, type_of_volume: QualifiedVolumeTypeValue) {
        self.type_of_volume = type_of_volume;
    }
}
