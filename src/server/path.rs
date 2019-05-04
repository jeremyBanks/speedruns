//! How do we want to handle URLs?
#![allow(unused)]

use std::{str::FromStr, sync::Arc};

use derive_more::{Display, From};
use err_derive::Error;
#[allow(unused)] use log::{debug, error, info, trace, warn};

use crate::{
    data::{
        database::{Database, Linked},
        types::*,
    },
    utils::slugify,
};

#[derive(Debug, From)]
pub enum Path {
    Home,
    User(Linked<User>),
    Game(Linked<Game>),
    FullCategory(Linked<Category>),
    LevelCategory(Linked<Category>, Linked<Level>),
    Run(Linked<Run>),
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Path::*;
        match self {
            Home => {
                write!(f, "/")?;
            }
            Game(game) => {
                write!(f, "/{}", game.slug())?;
            }
            User(user) => {
                write!(f, "/@{}", user.slug())?;
            }
            FullCategory(category) =>
                write!(f, "/{}/{}", category.game().slug(), category.slug())?,
            LevelCategory(category, level) => {
                write!(
                    f,
                    "/{}/{}/{}",
                    level.game().slug(),
                    category.slug(),
                    level.slug()
                )?;
            }
            Run(run) => {
                write!(f, "/{}/{}", run.game().slug(), run.category().slug())?;
                if let Some(level) = run.level() {
                    write!(f, "/{}", level.slug())?;
                }
                write!(f, "/{}", run.slug())?;
            }
        };
        Ok(())
    }
}

#[derive(Debug, Error, Display)]
pub enum PathParsingError {
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
    /// Returns the fully-qualified URL of a roughly-equivalent page on SpeedRun.Com.
    pub fn to_src(&self) -> String {
        // TODO: define Path::to_src() and use it in a view.
        unimplemented!();

        use Path::*;
        match self {
            Home => format!("https://speedrun.com/"),
            Game(game) => format!("https://speedrun.com/{}", game.src_slug()),
            User(user) => format!("https://speedrun.com/user/{}", user.src_slug()),
            FullCategory(category) => unimplemented!(),
            LevelCategory(category, level) => unimplemented!(),
            Run(run) => unimplemented!(),
        }
    }

    pub fn from_str(s: &str, database: Arc<Database>) -> Result<Path, PathParsingError> {
        debug!("got path: {}", s);
        assert!(s.starts_with('/'), "path must have leading slash");

        let pieces = &s[1..].split('/').collect::<Vec<_>>();

        let path = match pieces.len() {
            0 => Ok(Path::Home),
            3 => {
                let game_slug = slugify(&pieces[0]);
                let category_slug = slugify(&pieces[1]);
                let level_slug = slugify(&pieces[2]);

                let game = database.game_by_slug(&game_slug).unwrap();
                let category = game.category_by_slug(&category_slug).unwrap();
                let level = game.level_by_slug(&level_slug).unwrap();

                Ok(Path::LevelCategory(category, level))
            }
            _ => Err(PathParsingError::InvalidStructure),
        };

        let _ =
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

        The .json extension may be appended to any URL to return the view
        model as JSON instead of rendering it to HTML.

        Compatibility Redirects for SpeedRun.com Compatibility:
            /TGH                                       # user profile
            /user/TGH                                  # user profile
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
        ;

        if let Ok(ref path) = path {
            assert_eq!(s, path.to_string());
        }
        path
    }
}
