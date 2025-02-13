#[macro_export]
macro_rules! try_required {
    ($e:expr, $n:expr) => {
        $e.as_ref()
            .ok_or_else(|| Status::invalid_argument(format!("{} is required", $n)))
    };
}

#[macro_export]
macro_rules! resolve_error {
    ($e:expr) => {
        $e.map_err(|err| {
            error!(%err);
            Status::internal("Internal error")
        })
    };
}
