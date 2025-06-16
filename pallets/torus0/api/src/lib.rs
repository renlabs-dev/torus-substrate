#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate polkadot_sdk;

use core::str::FromStr;

use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    sp_core::ConstU32,
    sp_runtime::{BoundedVec, Percent},
    sp_std::vec::Vec,
};
use scale_info::TypeInfo;

/// The Torus0 pallet API.
pub trait Torus0Api<AccountId, Balance> {
    /// Interval of blocks in which rewards are distributed.
    fn reward_interval() -> u16;

    fn min_validator_stake() -> Balance;
    fn max_validators() -> u16;

    fn weight_control_fee(who: &AccountId) -> Percent;
    fn weight_penalty_factor(who: &AccountId) -> Percent;
    fn staking_fee(who: &AccountId) -> Percent;

    fn sum_staking_to(staker: &AccountId) -> Balance;

    fn staked_by(staked: &AccountId) -> alloc::vec::Vec<(AccountId, Balance)>;
    fn stake_to(staker: &AccountId, staked: &AccountId, amount: Balance) -> Result<(), Balance>;

    fn agent_ids() -> impl Iterator<Item = AccountId>;
    fn is_agent_registered(agent: &AccountId) -> bool;

    fn namespace_exists(agent: &AccountId, path: &NamespacePath) -> bool;

    #[doc(hidden)]
    #[cfg(feature = "runtime-benchmarks")]
    fn force_register_agent(
        id: &AccountId,
        name: alloc::vec::Vec<u8>,
        url: alloc::vec::Vec<u8>,
        metadata: alloc::vec::Vec<u8>,
    ) -> polkadot_sdk::frame_support::dispatch::DispatchResult;

    #[doc(hidden)]
    #[cfg(feature = "runtime-benchmarks")]
    fn force_set_stake(
        staker: &AccountId,
        staked: &AccountId,
        amount: Balance,
    ) -> polkadot_sdk::frame_support::dispatch::DispatchResult;
}

/// Number of total bytes a namespace path can contain.
/// This might have to increase in the future, but is a good enough default value.
/// If it ends up being formalized, the length can be described as a u8.
pub const MAX_NAMESPACE_PATH_LENGTH: usize = 256;
/// Number of total bytes a segment can contain.
pub const MAX_SEGMENT_LENGTH: usize = 64;
/// Max number of segments in a path.
pub const MAX_NAMESPACE_SEGMENTS: usize = 10;

pub const NAMESPACE_SEPARATOR: u8 = b'.';

pub type NamespacePathInner = BoundedVec<u8, ConstU32<{ MAX_NAMESPACE_PATH_LENGTH as u32 }>>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, PartialOrd, Ord, TypeInfo, MaxEncodedLen, Debug)]
pub struct NamespacePath(NamespacePathInner);

impl NamespacePath {
    /// Create a new namespace path from bytes with validation
    pub fn new(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.is_empty() {
            return Err("empty namespace path");
        }

        if bytes.len() > MAX_NAMESPACE_PATH_LENGTH {
            return Err("path too long");
        }

        let segments: Vec<&[u8]> = bytes.split(|&b| b == NAMESPACE_SEPARATOR).collect();
        if segments.len() > MAX_NAMESPACE_SEGMENTS {
            return Err("too many namespace segments");
        }

        for segment in &segments {
            let segment = core::str::from_utf8(segment).map_err(|_| "path is invalid itf-8")?;

            let first = segment.chars().next().ok_or("empty namespace segment")?;
            if !first.is_alphanumeric() {
                return Err("namespace segment must start with alphanumeric character");
            }

            if segment.len() > MAX_SEGMENT_LENGTH {
                return Err("namespace segment too long");
            }

            if segment
                .chars()
                .any(|c| !c.is_alphanumeric() && c != '-' && c != '_' && c != '+' && c != '=')
            {
                return Err("invalid character in namespace segment");
            }
        }

        bytes
            .to_vec()
            .try_into()
            .map(Self)
            .map_err(|_| "Failed to create bounded vec")
    }

    /// Converts this namespace into the inner bytes
    pub fn into_inner(self) -> NamespacePathInner {
        self.0
    }

    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Parse a namespace path into segments
    pub fn segments(&self) -> Vec<&[u8]> {
        self.as_bytes()
            .split(|&b| b == NAMESPACE_SEPARATOR)
            .collect()
    }

    /// Get the parent path of this namespace
    pub fn parent(&self) -> Option<Self> {
        let bytes = self.as_bytes();

        if let Some(last_dot) = bytes.iter().rposition(|&b| b == NAMESPACE_SEPARATOR) {
            bytes.get(..last_dot)?.to_vec().try_into().ok().map(Self)
        } else {
            None
        }
    }

    /// Get the depth of this namespace (number of segments)
    pub fn depth(&self) -> u32 {
        self.segments().len() as u32
    }

    /// Check if this path is a parent of another path
    pub fn is_parent_of(&self, child: &Self) -> bool {
        let self_bytes = self.as_bytes();
        let child_bytes = child.as_bytes();

        let Some(maybe_separator) = child_bytes.get(self_bytes.len()) else {
            return false;
        };

        child_bytes.starts_with(self_bytes) && *maybe_separator == NAMESPACE_SEPARATOR
    }

    /// Get all parent paths up to root
    pub fn parents(&self) -> Vec<Self> {
        let mut parents = Vec::new();
        let mut current = self.clone();

        while let Some(parent) = current.parent() {
            parents.push(parent.clone());
            current = parent;
        }

        parents
    }
}

impl FromStr for NamespacePath {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::indexing_slicing)]

    use super::*;

    #[test]
    fn namespace_creation_validates_paths() {
        assert!(NamespacePath::new(b"agent").is_ok());
        assert!(NamespacePath::new(b"agent.alice").is_ok());
        assert!(NamespacePath::new(b"agent.alice.memory").is_ok());
        assert!(NamespacePath::new(b"agent-1.alice_2.key=val+1").is_ok());

        assert!(NamespacePath::new(b"").is_err());
        assert!(NamespacePath::new(b".agent").is_err());
        assert!(NamespacePath::new(b"agent.").is_err());
        assert!(NamespacePath::new(b"agent..alice").is_err());
        assert!(NamespacePath::new(b"agent.-alice").is_err());
        assert!(NamespacePath::new(b"agent.alice!").is_err());
        assert!(NamespacePath::new(b"agent.alice memory").is_err());
    }

    #[test]
    fn namespace_segment_listing() {
        let path = NamespacePath::new(b"agent.alice.memory").unwrap();
        let segments = path.segments();
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0], b"agent");
        assert_eq!(segments[1], b"alice");
        assert_eq!(segments[2], b"memory");
    }

    #[test]
    fn namespace_parent_returns_correctly() {
        let path = NamespacePath::new(b"agent.alice.memory").unwrap();
        let parent = path.parent().unwrap();
        assert_eq!(parent.as_bytes(), b"agent.alice");

        let root = NamespacePath::new(b"agent").unwrap();
        assert!(root.parent().is_none());
    }

    #[test]
    fn namespace_depth_calculation() {
        let path1 = NamespacePath::new(b"agent").unwrap();
        assert_eq!(path1.depth(), 1);

        let path2 = NamespacePath::new(b"agent.alice.memory.twitter").unwrap();
        assert_eq!(path2.depth(), 4);
    }

    #[test]
    fn test_is_parent_of() {
        let parent = NamespacePath::new(b"agent.alice").unwrap();
        let child = NamespacePath::new(b"agent.alice.memory").unwrap();
        let other = NamespacePath::new(b"agent.bob").unwrap();

        assert!(parent.is_parent_of(&child));
        assert!(!parent.is_parent_of(&other));
        assert!(!child.is_parent_of(&parent));
    }

    #[test]
    fn test_parents() {
        let path = NamespacePath::new(b"agent.alice.memory.twitter").unwrap();
        let parents = path.parents();
        assert_eq!(parents.len(), 3);
        assert_eq!(parents[0].as_bytes(), b"agent.alice.memory");
        assert_eq!(parents[1].as_bytes(), b"agent.alice");
        assert_eq!(parents[2].as_bytes(), b"agent");
    }
}
