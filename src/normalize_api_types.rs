use crate::{api_types as api, normalized_types::*};

pub trait Normalize {
    type Normalized;
    type Error: std::error::Error = Box<dyn std::error::Error>;

    fn normalize(&self) -> Result<Self::Normalized, Self::Error>;
}

impl Normalize for api::User {
    type Normalized = User;

    fn normalize(&self) -> Result<Self::Normalized, Self::Error> {
        unimplemented!()
    }
}

impl Normalize for api::Game {
    type Normalized = (Game, Vec<Category>, Vec<Level>);

    fn normalize(&self) -> Result<Self::Normalized, Self::Error> {
        unimplemented!()
    }
}

impl Normalize for api::Run {
    type Normalized = Run;

    fn normalize(&self) -> Result<Self::Normalized, Self::Error> {
        unimplemented!()
    }
}
