use crate::database::object_mapper::ObjectMapper;

pub trait CanBuildObjectMapper {
    const TABLE_NAME: &'static str;
    fn build_object_mapper(&self) -> ObjectMapper;
}