//! Trusted data and nodes.

use crate::Hash;

/// Trusted node information.
#[derive(Debug, Clone)]
pub struct TrustedNode {
    pub id: Hash,
    pub is_trusted: bool,
}

impl TrustedNode {
    /// Creates a new trusted node.
    pub fn new(id: Hash, is_trusted: bool) -> Self {
        Self { id, is_trusted }
    }

    /// Checks if the node is trusted.
    pub fn is_trusted(&self) -> bool {
        self.is_trusted
    }
}

/// Trusted data set.
#[derive(Debug)]
pub struct TrustedData {
    pub nodes: Vec<TrustedNode>,
}

impl TrustedData {
    /// Creates new trusted data.
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    /// Adds a trusted node.
    pub fn add_node(&mut self, node: TrustedNode) {
        self.nodes.push(node);
    }

    /// Gets trusted nodes.
    pub fn trusted_nodes(&self) -> Vec<&TrustedNode> {
        self.nodes.iter().filter(|n| n.is_trusted).collect()
    }
}

impl Default for TrustedData {
    fn default() -> Self {
        Self::new()
    }
}

/// External ghostdag data.
#[derive(Debug, Clone, Default)]
pub struct ExternalGhostdagData {
    pub data: Vec<u8>,
}

/// Trusted block.
#[derive(Debug, Clone, Default)]
pub struct TrustedBlock {
    pub hash: Hash,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trusted_node() {
        let node = TrustedNode::new(Hash::default(), true);
        assert!(node.is_trusted());
    }

    #[test]
    fn test_trusted_data() {
        let mut data = TrustedData::new();
        let node = TrustedNode::new(Hash::default(), true);
        data.add_node(node);
        assert_eq!(data.trusted_nodes().len(), 1);
    }
}
