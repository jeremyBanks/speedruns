use std::{
    collections::{BTreeMap, HashMap},
    default::Default,
    fmt::{Debug, Display},
    ops::Deref,
    sync::Arc,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::From;
use err_derive::Error;
use getset::Getters;
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::{
    data::{models::*, types::*},
    utils::slugify,
};

/// Validates a Database for business and integrity requirements.
pub fn validate(_database: &super::Database) -> Result<(), IntegrityErrors> {
    Ok(())
}

#[derive(Debug, Error, From)]
pub struct IntegrityErrors {
    pub errors: Vec<IntegrityError>,
}

impl IntegrityErrors {
    // TODO: rename this!
    fn try_from(errors: Vec<IntegrityError>) -> Result<(), IntegrityErrors> {
        if errors.is_empty() {
            Ok(())
        } else {
            Err(IntegrityErrors { errors })
        }
    }
}

impl Display for IntegrityErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{} IntegrityErrors:", self.errors.len())?;
        for (i, error) in self.errors.iter().enumerate() {
            if i >= 16 {
                writeln!(f, "     ...and more!")?;
                break;
            }

            writeln!(f, "{:4}. {}", i + 1, error)?;
        }
        Ok(())
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error, From)]
pub enum IntegrityError {
    #[error(display = "integrity failure during indexing")]
    IndexingError,
    #[error(
        display = "{} with id {} does not exist, specified by {} in {:#?}",
        target_type,
        target_id,
        foreign_key_field,
        source
    )]
    ForeignKeyMissing {
        target_type: &'static str,
        target_id: u64,
        foreign_key_field: &'static str,
        source: AnyModel,
    },
    #[error(display = "row validation check failed: {:?} in {:?}", errors, source)]
    CheckFailed {
        errors: ValidationErrors,
        source: AnyModel,
    },
    #[error(display = "duplicate {:?} slug for {:?}", slug, sources)]
    NonUniqueSlug { slug: String, sources: AnyModelVec },
    #[error(display = "run is missing primary timing: {:?}", _0)]
    MissingPrimaryTiming(Run),
}
