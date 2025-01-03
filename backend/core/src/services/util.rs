use tonic::{Code, Status};
use tonic_types::{BadRequest, Help, StatusExt};

use crate::constants::*;

pub trait ExtendBadRequest {
    fn add_required(&mut self, cond: bool, name: &str);
    fn add_not_empty(&mut self, name: &str, data: &str);
    fn has_violation(self) -> Option<Status>;
}

// Blanket implementation for SomeStruct
impl ExtendBadRequest for BadRequest {
    #[tracing::instrument(level = "trace", skip(self, cond, name))]
    fn add_required(&mut self, cond: bool, name: &str) {
        if cond {
            self.add_violation(name, format!("{} is required", name));
        }
    }

    #[tracing::instrument(level = "trace", skip(self, name, data))]
    fn add_not_empty(&mut self, name: &str, data: &str) {
        if data.is_empty() {
            self.add_violation(name, format!("{} cannot be empty", name));
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn has_violation(self) -> Option<Status> {
        if !self.is_empty() {
            let help = Help::with_link("Project Repository", GITHUB_URL);

            return Some(Status::with_error_details_vec(
                Code::InvalidArgument,
                "request contains invalid arguments",
                vec![self.into(), help.into()],
            ));
        }
        None
    }
}
