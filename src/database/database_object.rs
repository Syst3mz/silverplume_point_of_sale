use crate::database::object_mapper::ObjectMapper;

pub trait DatabaseObject {
    fn build_object_mapper(&self) -> ObjectMapper;
}