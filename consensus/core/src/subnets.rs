//! Subnet utilities for network partitioning.

use crate::Hash;

/// Subnet identifier.
pub type SubnetId = u32;

/// Subnet information.
#[derive(Debug, Clone)]
pub struct Subnet {
    pub id: SubnetId,
    pub members: Vec<Hash>,
}

impl Subnet {
    /// Creates a new subnet.
    pub fn new(id: SubnetId) -> Self {
        Self { id, members: vec![] }
    }

    /// Adds a member to the subnet.
    pub fn add_member(&mut self, member: Hash) {
        self.members.push(member);
    }

    /// Checks if a member is in the subnet.
    pub fn has_member(&self, member: &Hash) -> bool {
        self.members.contains(member)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subnet() {
        let mut subnet = Subnet::new(1);
        let member = Hash::from_le_u64([1, 0, 0, 0]);
        subnet.add_member(member);
        assert!(subnet.has_member(&member));
    }
}
