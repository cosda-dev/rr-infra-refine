//! rac: Remote Actor Call.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FanAddress(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Aid(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChainDepth(pub u32);

pub trait RemoteActorCall {
    fn call(&mut self, target: FanAddress, aid: Aid, depth: ChainDepth, payload: &[u8]) -> RacResult;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RacResult(pub u64);
