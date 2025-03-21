use crate::smartauto::*;

pub trait AsType {
    fn as_type(&self) -> EntityType;
}

impl AsType for entity_value::Value {
    #[tracing::instrument(level = "trace", skip(self))]
    fn as_type(&self) -> EntityType {
        match self {
            entity_value::Value::Bool(_) => EntityType::Bool,
            entity_value::Value::Int(_) => EntityType::Int,
            entity_value::Value::Double(_) => EntityType::Double,
            entity_value::Value::String(_) => EntityType::String,
        }
    }
}
