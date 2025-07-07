pub trait ToModel {
    type ModelType;
    fn to_model(&self) -> anyhow::Result<Self::ModelType>;
}