//! moment: Logical time abstraction.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogicalTime(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AhrOnRef(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CausalStamp(pub u64);

pub trait TimeSource {
    fn now(&self) -> LogicalTime;
    fn stamp(&self) -> CausalStamp;
}
