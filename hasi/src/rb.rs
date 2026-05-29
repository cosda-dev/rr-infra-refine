//! rb: Resource Base (capability tokens instead of raw fds).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityToken(pub [u8; 32]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimSnapshotHandle(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CvedRef(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrustLevel {
    Low,
    Medium,
    High,
    Critical,
}

pub trait ResourceBase {
    fn resolve(&self, token: CapabilityToken) -> Option<IoHandleRef>;
    fn trust_level(&self, token: CapabilityToken) -> TrustLevel;
}

pub type IoHandleRef = hace_io::handle::IoHandle;
