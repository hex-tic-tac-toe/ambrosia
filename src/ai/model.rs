use crate::ai::genome::{FeatureSet, Genome};

pub struct Model<'a, T> {
    pub features: &'a FeatureSet<T>,
    pub genome: &'a Genome,
}

impl<'a, T> Model<'a, T> {
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
