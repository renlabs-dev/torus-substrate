use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    frame_support::{traits::ConstU32, CloneNoBound, DebugNoBound, PartialEqNoBound, EqNoBound},
    frame_system::pallet_prelude::BlockNumberFor,
    sp_runtime::BoundedVec,
};
use scale_info::TypeInfo;

use crate::Config;

/// Maximum length of a namespace path
pub const MAX_NAMESPACE_PATH_LENGTH: u32 = 256;

/// Maximum number of namespace segments
pub const MAX_NAMESPACE_SEGMENTS: u32 = 10;

/// Maximum length of a single namespace segment
pub const MAX_SEGMENT_LENGTH: u32 = 64;

/// Newtype wrapper for namespace paths with validation
#[derive(
    Encode,
    Decode,
    Clone,
    PartialEq,
    Eq,
    TypeInfo,
    MaxEncodedLen,
    Debug,
)]
pub struct NamespacePath(BoundedVec<u8, ConstU32<{ MAX_NAMESPACE_PATH_LENGTH }>>);

impl NamespacePath {
    /// Create a new namespace path from bytes with validation
    pub fn new(bytes: &[u8]) -> Result<Self, &'static str> {
        // Check length
        if bytes.is_empty() {
            return Err("Empty namespace path");
        }
        
        if bytes.len() > MAX_NAMESPACE_PATH_LENGTH as usize {
            return Err("Path too long");
        }
        
        // Validate segments
        let segments: Vec<&[u8]> = bytes.split(|&b| b == b'.').collect();
        
        if segments.len() > MAX_NAMESPACE_SEGMENTS as usize {
            return Err("Too many namespace segments");
        }
        
        for segment in &segments {
            if segment.is_empty() {
                return Err("Empty namespace segment");
            }
            
            if segment.len() > MAX_SEGMENT_LENGTH as usize {
                return Err("Namespace segment too long");
            }
            
            // Validate characters in segment (alphanumeric, hyphen, underscore)
            for &byte in segment.iter() {
                if !byte.is_ascii_alphanumeric() && byte != b'-' && byte != b'_' {
                    return Err("Invalid character in namespace segment");
                }
            }
            
            // Segment must start with alphanumeric
            if !segment[0].is_ascii_alphanumeric() {
                return Err("Namespace segment must start with alphanumeric character");
            }
        }
        
        // Create bounded vec
        let bounded = BoundedVec::<u8, ConstU32<{ MAX_NAMESPACE_PATH_LENGTH }>>::try_from(bytes.to_vec())
            .map_err(|_| "Failed to create bounded vec")?;
        
        Ok(Self(bounded))
    }
    
    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
    
    /// Parse a namespace path into segments
    pub fn segments(&self) -> Vec<&[u8]> {
        self.as_bytes().split(|&b| b == b'.').collect()
    }
    
    /// Get the parent path of this namespace
    pub fn parent(&self) -> Option<Self> {
        let bytes = self.as_bytes();
        if let Some(last_dot) = bytes.iter().rposition(|&b| b == b'.') {
            // Safe to create without validation since we know it was valid
            let bounded = BoundedVec::<u8, ConstU32<{ MAX_NAMESPACE_PATH_LENGTH }>>::try_from(
                bytes[..last_dot].to_vec()
            ).ok()?;
            Some(Self(bounded))
        } else {
            None
        }
    }
    
    /// Get the depth of this namespace (number of segments)
    pub fn depth(&self) -> u32 {
        self.segments().len() as u32
    }
    
    /// Check if this path is a parent of another path
    pub fn is_parent_of(&self, other: &Self) -> bool {
        let self_bytes = self.as_bytes();
        let other_bytes = other.as_bytes();
        
        if self_bytes.len() >= other_bytes.len() {
            return false;
        }
        
        other_bytes.starts_with(self_bytes) && other_bytes[self_bytes.len()] == b'.'
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

/// Metadata stored for each namespace
#[derive(
    Encode,
    Decode,
    CloneNoBound,
    PartialEqNoBound,
    EqNoBound,
    TypeInfo,
    MaxEncodedLen,
    DebugNoBound,
)]
#[scale_info(skip_type_params(T))]
pub struct NamespaceMetadata<T: Config> {
    /// Block number when the namespace was created
    pub created_at: BlockNumberFor<T>,
    /// Storage deposit paid for this namespace
    pub deposit: BalanceOf<T>,
}

pub type BalanceOf<T> = <<T as Config>::Currency as polkadot_sdk::frame_support::traits::Currency<
    <T as polkadot_sdk::frame_system::Config>::AccountId,
>>::Balance;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_namespace_creation() {
        // Valid paths
        assert!(NamespacePath::new(b"agent").is_ok());
        assert!(NamespacePath::new(b"agent.alice").is_ok());
        assert!(NamespacePath::new(b"agent.alice.memory").is_ok());
        assert!(NamespacePath::new(b"agent-1.alice_2").is_ok());
        
        // Invalid paths
        assert!(NamespacePath::new(b"").is_err());
        assert!(NamespacePath::new(b".agent").is_err());
        assert!(NamespacePath::new(b"agent.").is_err());
        assert!(NamespacePath::new(b"agent..alice").is_err());
        assert!(NamespacePath::new(b"agent.-alice").is_err());
        assert!(NamespacePath::new(b"agent.alice!").is_err());
        assert!(NamespacePath::new(b"agent.alice memory").is_err());
    }
    
    #[test]
    fn test_namespace_segments() {
        let path = NamespacePath::new(b"agent.alice.memory").unwrap();
        let segments = path.segments();
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0], b"agent");
        assert_eq!(segments[1], b"alice");
        assert_eq!(segments[2], b"memory");
    }
    
    #[test]
    fn test_namespace_parent() {
        let path = NamespacePath::new(b"agent.alice.memory").unwrap();
        let parent = path.parent().unwrap();
        assert_eq!(parent.as_bytes(), b"agent.alice");
        
        let root = NamespacePath::new(b"agent").unwrap();
        assert!(root.parent().is_none());
    }
    
    #[test]
    fn test_namespace_depth() {
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