//! How do we want to handle URLs?
#![allow(unused)]

use std::{rc::Rc, str::FromStr};

use derive_more::Display;
use err_derive::Error;

use crate::data::{
    database::{Database, Linked},
    types::*,
};

#[derive(Debug)]
enum Path {
    Home,
    User(Linked<User>),
    Game(Linked<Game>),
    FullCategory(Linked<Category>),
    LevelCategory(Linked<Category>, Linked<Level>),
    FullRun(Linked<Run>),
    LevelRun(Linked<Run>),
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Path::*;
        match self {
            Home => write!(f, "/"),
            Game(game) => write!(f, "/{}", game.slug()),
            User(user) => write!(f, "/@{}", user.slug()),
            FullCategory(category) =>
                write!(f, "/{}/{}", category.game().slug(), category.slug()),
            LevelCategory(category, level) => write!(
                f,
                "/{}/{}/{}",
                level.game().slug(),
                category.slug(),
                level.slug()
            ),
            FullRun(run) => write!(
                f,
                "/{}/{}/{}",
                run.game().slug(),
                run.category().slug(),
                run.slug()
            ),
            LevelRun(run) => write!(
                f,
                "/{}/{}/{}/{}",
                run.game().slug(),
                run.category().slug(),
                run.level().expect("LevelRun run must have level").slug(),
                run.slug()
            ),
        }
    }
}

#[derive(Debug, Error, Display)]
enum PathParsingError {
    /// The provided path wasn't in the required format, but were able to
    /// normalize it.
    /// 301 temporary redirect
    #[display(
        fmt = "URL was not normalized, but we were able to normalize it: {:?}",
        _0
    )]
    Found(Path),
    /// We weren't able to find a record matching some component of the path.
    /// 404 not found
    #[display(fmt = "item could not be found")]
    NotFound,
    /// The format of the path was invalid in some value-independent way.
    /// 400 client error
    #[display(fmt = "path structure invalid")]
    InvalidStructure,
}

impl Path {
    fn from_str(s: &str, database: Rc<Database>) -> Result<Path, PathParsingError> {
        let path: Result<Path, PathParsingError> = unimplemented!(
        r"
        Our URLs:
            /                                          # game list
            /@tgh                                      # profile profile (just redirect to SRC)
            /celeste                                   # game category list
            /celeste/anypercent                        # full and/or level category
            /celeste/anypercent/tgh-y6p1               # user full run
            /celeste/anypercent/y6p1yj4w               # guest full run
            /celeste/anypercent/forsaken-city          # level in category
            /celeste/anypercent/forsaken-city/pac-yj4w # user level run

        If slug components of the string aren't already normalized (slugified)
        we redirect the user to the normalized form.

        TODO: The .json extension may be appended to any URL to return the view
        model as JSON instead of rendering it to HTML.

        Compatibility Redirects for SpeedRun.com Compatibility:
            /TGH                                       # user profile
            /Celeste/full_game                         # game category list
            /Celeste/individual_levels                 # game category list
            /Celeste/Forsaken_City                     # game category list
            /Celeste/run/zp0jldgm                      # run

        Convenience Redirects:
            /zp0jldgm                                  # any base 36 ID
            /1884988586418                             # any base 10 ID
            /forsaken-city                             # any slug
            /Forsaken%20City                           # (remember that we normalize them)
            /celeste/anypercent/tgh                    # personal best full run
            /celeste/anypercent/forsaken-city/pac      # personal best level run
        "
        );

        if let Ok(path) = path {
            assert_eq!(s, path.to_string());
        }
        path
    }
}
