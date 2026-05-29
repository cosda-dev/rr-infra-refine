//! tube: Deterministic message transport.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RateLimit(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Priority {
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RcMStage {
    Observe,
    Warn,
    Throttle,
    Suspend,
    Quarantine,
    Escalate,
}

pub trait Tube {
    fn send(&mut self, id: MessageId, priority: Priority, payload: &[u8]) -> bool;
    fn rate_limit(&self) -> RateLimit;
}
