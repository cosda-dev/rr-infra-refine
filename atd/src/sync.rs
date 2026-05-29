//! atd::sync
//! Authority-bound synchronization primitives.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthorityId(u64);

impl AuthorityId {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}
