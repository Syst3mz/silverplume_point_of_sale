pub trait HasTotalCost {
    fn total_cost(&self) -> f32;
}

impl<'a, T: HasTotalCost + 'a, I> HasTotalCost for &'a I where &'a I: IntoIterator<Item = &'a T>, T: HasTotalCost {
    fn total_cost(&self) -> f32 {
        self.into_iter().map(|x| x.total_cost()).sum()
    }
}