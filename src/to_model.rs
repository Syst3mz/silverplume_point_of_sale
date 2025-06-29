pub trait ToModel<T> {
    fn to_model(&self) -> anyhow::Result<T>;
}