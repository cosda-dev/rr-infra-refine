use alloc::vec::Vec;

use super::instance::WasmModuleRef;

pub struct WasmExecutor;

impl WasmExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, _module: &WasmModuleRef, _input: &[u8]) -> Vec<u8> {
        // IPO: refine execution stub
        Vec::new()
    }
}
