//! atd::io
//! IO is authority-mediated (no direct syscalls).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IoHandle(u64);

impl IoHandle {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}
