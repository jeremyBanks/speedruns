#[allow(unused)]
use log::{debug, error, info, trace, warn};

pub type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    env_logger::try_init_from_env(
        env_logger::Env::new()
            .default_filter_or(format!("{}=trace", module_path!())),
    )?;

    Ok(())
}
