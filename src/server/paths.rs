use std::str::FromStr;

use derive_more::Display;

use crate::{
    data::{database::Linked, types::*},
    utils::{base36, slugify},
};

#[derive(Debug)]
pub enum Path {
    Home,
    Game(Linked<Game>),
    User(Linked<User>),
    Category(Linked<Category>),
    Level(Linked<Level>),
}

impl FromStr for Path {
    type Err = Box<dyn std::error::Error>;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unimplemented!();

        r"

/@tgh
/celeste/
/celeste/anypercent  
/celeste/anypercent/y6p19j6m
/celeste/anypercent
/celeste/anypercent/forsaken-city
/celeste/anypercent/forsaken-city/yj4wr8dy

+obsolete


";
    }
}
