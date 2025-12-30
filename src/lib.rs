pub mod gas;
pub mod simulator;
pub mod adapter;

pub use gas::{GasCosts, GasConfig, GasMeter};
pub use adapter::{ContractState, BlockchainAdapter, SimulatorAdapter, SolanaAdapter, SolanaAccount, SolanaContext};

/// Runtime version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
