use std::sync::Arc;

use crate::ai::feature::Feature;

pub struct Genome {
    pub weights: Vec<f64>,
}

pub struct FeatureSet<T> {
    pub features: Vec<Arc<dyn Feature<T>>>,
}

impl<T> FeatureSet<T> {
    pub fn len(&self) -> usize {
        self.features.len()
    }
}
