use regex::Regex;
use tonic::{Code, Status};
use tonic_types::{BadRequest, Help, StatusExt};

use crate::constants::*;
use crate::smartauto::*;

pub trait ExtendBadRequest {
    fn add_required<T>(&mut self, name: &str, data: &Option<T>);
    fn add_not_empty(&mut self, name: &str, data: &str);
    fn add_not_valid_id(&mut self, name: &str, data: &str);
    fn add_not_valid_type(&mut self, name: &str, data: i32);
    fn has_violation(self) -> Result<(), Status>;
}

// Blanket implementation for SomeStruct
impl ExtendBadRequest for BadRequest {
    #[tracing::instrument(level = "trace", skip(self, name, data))]
    fn add_required<T>(&mut self, name: &str, data: &Option<T>) {
        if data.is_none() {
            self.add_violation(name, format!("{} is required", name));
        }
    }

    #[tracing::instrument(level = "trace", skip(self, name, data))]
    fn add_not_empty(&mut self, name: &str, data: &str) {
        if data.is_empty() {
            self.add_violation(name, format!("{} cannot be empty", name));
        }
    }

    #[tracing::instrument(level = "trace", skip(self, name, data))]
    fn add_not_valid_id(&mut self, name: &str, data: &str) {
        self.add_not_empty(name, &data);

        let entity_id_regex = Regex::new(ENTITY_ID_REGEX).unwrap();

        if !entity_id_regex.is_match(&data) {
            self.add_violation(
                name,
                format!(
                    "{} is not an valid entity identifier! It has to match \"{}\"",
                    name, ENTITY_ID_REGEX
                ),
            );
        }
    }

    #[tracing::instrument(level = "trace", skip(self, name, data))]
    fn add_not_valid_type(&mut self, name: &str, data: i32) {
        if let Ok(entity) = EntityType::try_from(data) {
            if entity == EntityType::Unspecified {
                self.add_violation(name, format!("{} has to bet set", name));
            }
        } else {
            self.add_violation(name, format!("{} unknown enum value", name));
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn has_violation(self) -> Result<(), Status> {
        if !self.is_empty() {
            let help = Help::with_link("Project Repository", GITHUB_URL);

            return Err(Status::with_error_details_vec(
                Code::InvalidArgument,
                "request contains invalid arguments",
                vec![self.into(), help.into()],
            ));
        }
        Ok(())
    }
}
