//! atd::panic
//! Panic is escalation; no cross-authority unwind.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EscalationCode(u32);

impl EscalationCode {
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }
}
