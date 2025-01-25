use regex::Regex;
use tonic::{Code, Status};
use tonic_types::{BadRequest, Help, StatusExt};

use crate::constants::*;
use crate::smartauto::*;

pub trait ExtendBadRequest {
    fn validate_type(&mut self, name: &str, data: i32);
    fn validate_not_empty(&mut self, name: &str, data: &str) -> bool;
    fn validate_id(&mut self, name: &str, data: &str) -> bool;

    fn has_violation(self) -> Result<(), Status>;
}

// Blanket implementation for SomeStruct
impl ExtendBadRequest for BadRequest {
    #[tracing::instrument(level = "trace", skip(self, name, data))]
    fn validate_type(&mut self, name: &str, data: i32) {
        if let Ok(entity) = EntityType::try_from(data) {
            if entity == EntityType::Unspecified {
                self.add_violation(name, format!("{} has to bet set", name));
            }
        } else {
            self.add_violation(name, format!("{} unknown enum value", name));
        }
    }

    #[tracing::instrument(level = "trace", skip(self, name))]
    fn validate_not_empty(&mut self, name: &str, data: &str) -> bool {
        let is_empty = data.is_empty();
        if is_empty {
            self.add_violation(name, format!("{} cannot be empty", name));
        }
        is_empty
    }

    #[tracing::instrument(level = "trace", skip(self, name))]
    fn validate_id(&mut self, name: &str, data: &str) -> bool {
        self.validate_not_empty(name, data);

        let entity_id_regex = Regex::new(ENTITY_ID_REGEX).unwrap();

        if !entity_id_regex.is_match(data) {
            self.add_violation(
                name,
                format!(
                    "{} is not an valid entity identifier! It has to match \"{}\"",
                    name, ENTITY_ID_REGEX
                ),
            );
            return false;
        }
        true
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
