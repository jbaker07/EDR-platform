// src/features.rs
#[derive(Clone, Debug)]
pub struct FeatureObservation {
    pub name: String,
    pub value: String,
}

impl FeatureObservation {
    pub fn new<N: Into<String>, V: Into<String>>(name: N, value: V) -> Self {
        Self { name: name.into(), value: value.into() }
    }
}
