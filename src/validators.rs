use validator::{Validate, ValidationError, ValidationErrors};

pub fn urls(value: &[String]) -> Result<(), ValidationError> {
    for item in value {
        if !validator::validate_url(item) {
            return Err(ValidationError::new("invalid URL"))
        }
    }
    Ok(())
}

pub trait IsDefault: Default + PartialEq {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl IsDefault for &str {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

impl<T> IsDefault for Vec<T>
where
    T: PartialEq,
{
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

pub fn nondefault(value: impl IsDefault) -> Result<(), ValidationError> {
    if value.is_default() {
        Err(ValidationError::new("value is uninitialized"))
    } else {
        Ok(())
    }
}
