pub mod gas;

pub use gas::{GasCosts, GasConfig, GasMeter};

/// Runtime version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
