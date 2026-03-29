/// Some feature about `T`.
pub trait Feature<T> {
    fn name(&self) -> &'static str;
    fn score(&self, item: &T) -> f64;
}
