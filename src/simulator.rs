use crate::adapter::ContractState;
use anyhow::{anyhow, Result};
use wasmi::{Caller, Engine, Linker, Module, Store};

pub struct ContractSimulator {
    engine: Engine,
    linker: Linker<ContractState>,
}

impl Default for ContractSimulator {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractSimulator {
    pub fn new() -> Self {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        // Host functions
        linker
            .func_wrap(
                "env",
                "get_caller",
                |caller: Caller<'_, ContractState>| -> i64 { caller.data().caller },
            )
            .expect("failed to wrap get_caller");

        linker
            .func_wrap(
                "env",
                "get_value",
                |caller: Caller<'_, ContractState>| -> i64 { caller.data().value },
            )
            .expect("failed to wrap get_value");

        linker
            .func_wrap(
                "env",
                "get_data",
                |caller: Caller<'_, ContractState>| -> i64 { caller.data().data },
            )
            .expect("failed to wrap get_data");

        linker
            .func_wrap(
                "env",
                "storage_read",
                |caller: Caller<'_, ContractState>, key: i64| -> i64 {
                    *caller.data().storage.get(&key).unwrap_or(&0)
                },
            )
            .expect("failed to wrap storage_read");

        linker
            .func_wrap(
                "env",
                "storage_write",
                |mut caller: Caller<'_, ContractState>, key: i64, value: i64| {
                    caller.data_mut().storage.insert(key, value);
                },
            )
            .expect("failed to wrap storage_write");

        linker
            .func_wrap(
                "env",
                "emit_event",
                |mut caller: Caller<'_, ContractState>, id: i64, data: i64| {
                    println!("EVENT EMITTED: id={}, data={}", id, data);
                    caller.data_mut().events.push((id, data));
                },
            )
            .expect("failed to wrap emit_event");

        linker
            .func_wrap(
                "env",
                "hash_i64",
                |_: Caller<'_, ContractState>, a: i64, b: i64| -> i64 {
                    // A robust non-cryptographic hash (splitmix64 style)
                    let mut x = (a as u64)
                        .wrapping_add(b as u64)
                        .wrapping_add(0x9E3779B97F4A7C15);
                    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
                    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
                    x = x ^ (x >> 31);
                    x as i64
                },
            )
            .expect("failed to wrap hash_i64");

        Self { engine, linker }
    }

    pub fn run(
        &self,
        wasm_bytes: &[u8],
        func_name: &str,
        args: &[i64],
        state: ContractState,
    ) -> Result<(i64, ContractState)> {
        let module = Module::new(&self.engine, wasm_bytes).map_err(|e| {
            let hex = wasm_bytes
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<String>>()
                .join("");
            let _ = std::fs::write("/tmp/contract.wasm.hex", &hex);
            anyhow!(
                "Failed to load module: {}\nWASM hex saved to /tmp/contract.wasm.hex",
                e
            )
        })?;
        let mut store = Store::new(&self.engine, state);

        let instance = self
            .linker
            .instantiate(&mut store, &module)
            .map_err(|e| anyhow!("Failed to instantiate module: {}", e))?
            .start(&mut store)
            .map_err(|e| anyhow!("Failed to start module: {}", e))?;

        let func = instance
            .get_export(&store, func_name)
            .and_then(|e| e.into_func())
            .ok_or_else(|| anyhow!("Function '{}' not found in WASM", func_name))?;

        let wasmi_args: Vec<wasmi::Value> = args.iter().map(|&a| wasmi::Value::I64(a)).collect();
        let func_ty = func.ty(&store);
        let mut results = vec![wasmi::Value::I64(0); func_ty.results().len()];

        func.call(&mut store, &wasmi_args, &mut results)
            .map_err(|e| anyhow!("Error calling function: {}", e))?;

        let res = if let Some(wasmi::Value::I64(val)) = results.first() {
            *val
        } else {
            0
        };

        Ok((res, store.into_data()))
    }
}
