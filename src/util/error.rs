use std::error::Error;

pub type DynError = Box<dyn Error>;

pub type DynResult<T> = Result<T, DynError>;