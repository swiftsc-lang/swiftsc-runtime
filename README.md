# SwiftSC-Lang Runtime

This directory is reserved for the blockchain runtime implementation.

## Current Implementation

Runtime features are currently in:
- `/swiftsc-compiler/swiftsc-backend/src/gas.rs` - Gas metering
- Host function definitions in codegen

## Features

- Gas cost model
- Resource limits
- Host function interfaces
- Storage operations
- Event emission

## Future

This directory may contain:
- Standalone WASM runtime
- Blockchain adapters
- Runtime testing framework
