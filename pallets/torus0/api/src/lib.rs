#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate polkadot_sdk;

use core::{
    fmt::{Debug, Display},
    str::FromStr,
};

use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    sp_core::ConstU32,
    sp_runtime::{BoundedVec, Percent},
    sp_std::vec::Vec,
};
use scale_info::TypeInfo;

pub mod api;

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
/// Number of total bytes a segment can contain. 63 plus dot.
pub const MAX_SEGMENT_LENGTH: usize = 63;
/// Max number of segments in a path. In the common prefix case, an agent will have
/// up to 8 levels of depth to use, 2 being allocated to the agent prefix notation.
pub const MAX_NAMESPACE_SEGMENTS: usize = 10;

pub const NAMESPACE_SEPARATOR: u8 = b'.';
pub const NAMESPACE_AGENT_PREFIX: &[u8] = b"agent.";

pub type NamespacePathInner = BoundedVec<u8, ConstU32<{ MAX_NAMESPACE_PATH_LENGTH as u32 }>>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, PartialOrd, Ord, TypeInfo, MaxEncodedLen)]
pub struct NamespacePath(NamespacePathInner);

impl NamespacePath {
    /// The root agent namespace entry.
    pub fn agent_root() -> NamespacePath {
        NamespacePath(b"agent".to_vec().try_into().unwrap())
    }

    /// Create a new root agent namespace path from the agent name
    pub fn new_agent_root(agent_name: &[u8]) -> Result<Self, &'static str> {
        let namespace_path: Vec<_> = [NAMESPACE_AGENT_PREFIX, agent_name].concat();
        Self::new_agent(&namespace_path)
    }

    /// Create a new namespace path from bytes with validation
    pub fn new_agent(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.is_empty() {
            return Err("empty namespace path");
        }

        if bytes.len() > MAX_NAMESPACE_PATH_LENGTH {
            return Err("path too long");
        }

        if !bytes.starts_with(NAMESPACE_AGENT_PREFIX) {
            return Err("path must begin with agent prefix");
        }

        let segments: Vec<&[u8]> = bytes.split(|&b| b == NAMESPACE_SEPARATOR).collect();
        if segments.len() > MAX_NAMESPACE_SEGMENTS {
            return Err("too many namespace segments");
        }

        for segment in &segments {
            let segment = core::str::from_utf8(segment).map_err(|_| "path is invalid itf-8")?;

            if segment.len() > MAX_SEGMENT_LENGTH {
                return Err("namespace segment too long");
            }

            let first = segment.chars().next().ok_or("empty namespace segment")?;
            let last = segment.chars().last().ok_or("empty namespace segment")?;
            if !first.is_ascii_alphanumeric() || !last.is_ascii_alphanumeric() {
                return Err("namespace segment must start and end with alphanumeric characters");
            }

            if segment.chars().any(|c| {
                !(c.is_ascii_digit() || c.is_ascii_lowercase()) && c != '-' && c != '_' && c != '+'
            }) {
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
    pub fn segments(&self) -> impl Iterator<Item = &[u8]> {
        self.as_bytes().split(|&b| b == NAMESPACE_SEPARATOR)
    }

    /// Returns the first segment of the path.
    pub fn root(&self) -> Option<Self> {
        let Some(root) = self.as_bytes().split(|&b| b == NAMESPACE_SEPARATOR).next() else {
            return Some(self.clone());
        };

        root.to_vec().try_into().ok().map(Self)
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
        self.segments().count() as u32
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

    /// Wether the namespace is an agent root path: "agent.<name>".
    pub fn is_agent_root(&self) -> bool {
        if self.depth() != 2 {
            return false;
        }

        if let Some(root) = self.segments().next() {
            root == b"agent"
        } else {
            false
        }
    }
}

impl Debug for NamespacePath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NamespacePath")
            .field(&core::str::from_utf8(&self.0))
            .finish()
    }
}

impl Display for NamespacePath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(&self.0).unwrap_or("invalid path"))
    }
}

impl FromStr for NamespacePath {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new_agent(s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::indexing_slicing)]

    use super::*;

    #[test]
    fn namespace_creation_validates_paths() {
        assert!(NamespacePath::new_agent(b"agent.alice").is_ok());
        assert!(NamespacePath::new_agent(b"agent.alice_2.memory-1.func+1").is_ok());

        assert!(NamespacePath::new_agent(format!("agent.alice.{:0<63}", 1).as_bytes()).is_ok());
        assert!(NamespacePath::new_agent(format!("agent.alice.{:0<64}", 1).as_bytes()).is_err());

        assert!(NamespacePath::new_agent(b"").is_err());
        assert!(NamespacePath::new_agent(b"agent").is_err());
        assert!(NamespacePath::new_agent(b".agent").is_err());
        assert!(NamespacePath::new_agent(b"agent.").is_err());
        assert!(NamespacePath::new_agent(b"agent.Alice").is_err());
        assert!(NamespacePath::new_agent(b"agent..alice").is_err());
        assert!(NamespacePath::new_agent(b"agent.-alice").is_err());
        assert!(NamespacePath::new_agent(b"agent.alice-").is_err());
        assert!(NamespacePath::new_agent(b"agent.-alice-").is_err());
        assert!(NamespacePath::new_agent(b"agent.alice!").is_err());
        assert!(NamespacePath::new_agent(b"agent.alice memory").is_err());
        assert!(NamespacePath::new_agent("agent.alice.tørûs".as_bytes()).is_err());
    }

    #[test]
    fn namespace_segment_listing() {
        let path = NamespacePath::new_agent(b"agent.alice.memory").unwrap();
        let mut segments = path.segments();
        assert_eq!(segments.next(), Some(b"agent".as_slice()));
        assert_eq!(segments.next(), Some(b"alice".as_slice()));
        assert_eq!(segments.next(), Some(b"memory".as_slice()));
        assert_eq!(segments.next(), None);
    }

    #[test]
    fn namespace_parent_returns_correctly() {
        let path = NamespacePath::new_agent(b"agent.alice.memory").unwrap();
        let parent = path.parent().unwrap();
        assert_eq!(parent.as_bytes(), b"agent.alice");

        let root = NamespacePath::agent_root();
        assert!(root.parent().is_none());
    }

    #[test]
    fn namespace_depth_calculation() {
        let path1 = NamespacePath::new_agent(b"agent.alice").unwrap();
        assert_eq!(path1.depth(), 2);

        let path2 = NamespacePath::new_agent(b"agent.alice.memory.twitter").unwrap();
        assert_eq!(path2.depth(), 4);
    }

    #[test]
    fn test_is_parent_of() {
        let parent = NamespacePath::new_agent(b"agent.alice").unwrap();
        let child = NamespacePath::new_agent(b"agent.alice.memory").unwrap();
        let other = NamespacePath::new_agent(b"agent.bob").unwrap();

        assert!(parent.is_parent_of(&child));
        assert!(!parent.is_parent_of(&other));
        assert!(!child.is_parent_of(&parent));
    }

    #[test]
    fn test_parents() {
        let path = NamespacePath::new_agent(b"agent.alice.memory.twitter").unwrap();
        let parents = path.parents();
        assert_eq!(parents.len(), 3);
        assert_eq!(parents[0].as_bytes(), b"agent.alice.memory");
        assert_eq!(parents[1].as_bytes(), b"agent.alice");
        assert_eq!(parents[2].as_bytes(), b"agent");
    }

    #[test]
    fn test_is_agent_root() {
        let agent_alice = NamespacePath::new_agent(b"agent.alice").unwrap();
        assert!(agent_alice.is_agent_root());

        let agent_bob = NamespacePath::new_agent(b"agent.bob").unwrap();
        assert!(agent_bob.is_agent_root());

        let deeper = NamespacePath::new_agent(b"agent.alice.memory").unwrap();
        assert!(!deeper.is_agent_root());

        let just_agent = NamespacePath::agent_root();
        assert!(!just_agent.is_agent_root());
    }

    #[test]
    fn test_new_agent_root() {
        let alice_root = NamespacePath::new_agent_root(b"alice").unwrap();
        assert_eq!(alice_root.as_bytes(), b"agent.alice");
        assert!(alice_root.is_agent_root());

        let bob_root = NamespacePath::new_agent_root(b"bob").unwrap();
        assert_eq!(bob_root.as_bytes(), b"agent.bob");
        assert!(bob_root.is_agent_root());

        assert!(NamespacePath::new_agent_root(b"alice123").is_ok());
        assert!(NamespacePath::new_agent_root(b"alice-test").is_ok());
        assert!(NamespacePath::new_agent_root(b"alice_test").is_ok());

        assert!(
            NamespacePath::new_agent_root(b"Alice").is_err(),
            "uppercase should fail"
        );
        assert!(
            NamespacePath::new_agent_root(b"alice!").is_err(),
            "special chars should fail"
        );
        assert!(
            NamespacePath::new_agent_root(b"").is_err(),
            "empty name should fail"
        );
    }
}
