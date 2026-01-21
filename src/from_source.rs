use std::error::Error;

pub trait FromSource {
    fn from_source(source: &str) -> Result<Self, &'static str>;
}