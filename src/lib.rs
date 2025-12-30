pub mod adapter;
pub mod gas;
pub mod simulator;

pub use adapter::{
    BlockchainAdapter, ContractState, SimulatorAdapter, SolanaAccount, SolanaAdapter, SolanaContext,
};
pub use gas::{GasConfig, GasCosts, GasMeter};

/// Runtime version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
