use crate::database::object_mapper::ObjectMapper;

pub trait CanBuildObjectMapper {
    fn build_object_mapper(&self) -> ObjectMapper;
}