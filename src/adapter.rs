use std::collections::HashMap;
use anyhow::Result;
use crate::simulator::ContractSimulator;

#[derive(Debug, Clone, Default)]
pub struct SolanaAccount {
    pub address: i64,
    pub owner: i64,
    pub data: Vec<u8>,
    pub lamports: u64,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SolanaContext {
    pub accounts: Vec<SolanaAccount>,
    pub program_id: i64,
}

#[derive(Debug, Clone, Default)]
pub struct ContractState {
    pub storage: HashMap<i64, i64>,
    pub caller: i64,
    pub value: i64,
    pub data: i64, // For now, treat data as an i64 (could be ptr later)
    pub events: Vec<(i64, i64)>,
    pub solana: Option<SolanaContext>,
}

pub trait BlockchainAdapter {
    fn execute(
        &self,
        wasm_bytes: &[u8],
        func_name: &str,
        args: &[i64],
        state: ContractState,
    ) -> Result<(i64, ContractState)>;
}

pub struct SimulatorAdapter {
    simulator: ContractSimulator,
}

impl SimulatorAdapter {
    pub fn new() -> Self {
        Self {
            simulator: ContractSimulator::new(),
        }
    }
}

impl BlockchainAdapter for SimulatorAdapter {
    fn execute(
        &self,
        wasm_bytes: &[u8],
        func_name: &str,
        args: &[i64],
        state: ContractState,
    ) -> Result<(i64, ContractState)> {
        self.simulator.run(wasm_bytes, func_name, args, state)
    }
}

pub struct SolanaAdapter {
    simulator: ContractSimulator,
}

impl SolanaAdapter {
    pub fn new() -> Self {
        Self {
            simulator: ContractSimulator::new(),
        }
    }
}

impl BlockchainAdapter for SolanaAdapter {
    fn execute(
        &self,
        wasm_bytes: &[u8],
        func_name: &str,
        args: &[i64],
        state: ContractState,
    ) -> Result<(i64, ContractState)> {
        // Conceptual Solana validation
        if let Some(ctx) = &state.solana {
            println!("--- Solana Conceptual Execution ---");
            println!("Program ID: {}", ctx.program_id);
            println!("Accounts: {}", ctx.accounts.len());
            
            for acc in &ctx.accounts {
                if acc.is_writable && acc.owner != ctx.program_id {
                    return Err(anyhow::anyhow!("Cannot write to account {} not owned by program", acc.address));
                }
            }
        }

        // Run simulation
        self.simulator.run(wasm_bytes, func_name, args, state)
    }
}
