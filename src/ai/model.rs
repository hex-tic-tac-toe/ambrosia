use std::{collections::HashMap, sync::Arc};

use crate::ai::{
    feature::Feature,
    genome::{FeatureSet, Genome},
};

pub struct Model<T> {
    pub features: FeatureSet<T>,
    pub genome: Genome,
}

impl<T> Model<T> {
    pub fn create(map: HashMap<Arc<dyn Feature<T>>, f64>) -> (FeatureSet<T>, Genome) {
        let features: Vec<Arc<dyn Feature<T>>> = map.keys().cloned().collect();
        let weights: Vec<f64> = map.values().cloned().collect();
        (FeatureSet { features }, Genome { weights })
    }

    pub fn new(features: FeatureSet<T>, genome: Genome) -> Self {
        Self { features, genome }
    }

    pub fn evaluate(&self, state: &T) -> f64 {
        self.features
            .features
            .iter()
            .zip(&self.genome.weights)
            .map(|(f, w)| f.score(state) * w)
            .sum()
    }
}

pub trait Fitness<T> {
    fn score(&self, model: &Model<T>) -> f64;
}
