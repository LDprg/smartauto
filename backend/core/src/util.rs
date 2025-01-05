use crate::smartauto::*;

pub trait AsType {
    fn as_type(&self) -> EntityType;
}

impl AsType for entity_value::Value {
    fn as_type(&self) -> EntityType {
        match self {
            entity_value::Value::Bool(_) => EntityType::Bool,
            entity_value::Value::Int(_) => EntityType::Int,
            entity_value::Value::Float(_) => EntityType::Float,
            entity_value::Value::String(_) => EntityType::String,
        }
    }
}
