use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct WasmModuleRef {
    pub id: [u8; 32],
    pub bytes: Vec<u8>,
}
