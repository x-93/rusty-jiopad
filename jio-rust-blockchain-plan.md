# JIO Blockchain - Rust Implementation Plan
## High-Performance GhostDAG Protocol with HeavyHash POW

---

## Executive Summary

A comprehensive Rust implementation plan for JIO blockchain, leveraging Rust's memory safety, concurrency features, and performance characteristics. This plan builds upon modern Rust blockchain patterns while implementing GhostDAG consensus with HeavyHash mining algorithm.

---

## 1. Project Structure & Architecture

### Repository Structure
```
rusty-jio/
├── Cargo.toml                 # Workspace configuration
├── crates/
│   ├── jio-core/              # Core blockchain primitives
│   ├── jio-consensus/         # GhostDAG consensus implementation
│   ├── jio-crypto/            # Cryptographic primitives
│   ├── jio-mining/            # HeavyHash mining implementation
│   ├── jio-network/           # P2P networking layer
│   ├── jio-rpc/               # RPC server implementation
│   ├── jio-storage/           # Database and storage layer
│   ├── jio-wallet/            # Wallet implementation
│   ├── jio-utils/             # Shared utilities
│   └── jio-node/              # Full node implementation
├── benches/                   # Performance benchmarks
├── tests/                     # Integration tests
├── scripts/                   # Build and deployment scripts
└── docs/                      # Documentation

```

### Core Dependencies
```toml
[workspace]
members = [
    "crates/*"
]

[workspace.dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"

# Serialization
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
borsh = "1.3"

# Cryptography
blake3 = "1.5"
sha3 = "0.10"
secp256k1 = "0.28"
ed25519-dalek = "2.1"
rand = "0.8"

# Database
rocksdb = "0.21"
sled = "0.34"

# Networking
libp2p = "0.53"
quinn = "0.10"  # QUIC protocol
tonic = "0.11"  # gRPC

# Utilities
thiserror = "1.0"
anyhow = "1.0"
tracing = "0.1"
clap = { version = "4.4", features = ["derive"] }
config = "0.13"
chrono = "0.4"
num-bigint = "0.4"
rayon = "1.8"  # Parallel processing
dashmap = "5.5"  # Concurrent hashmap
parking_lot = "0.12"  # Better mutex
crossbeam = "0.8"  # Lock-free data structures
```

---

## 2. Core Data Structures (Rust Implementation)

### Block & Header Structures
```rust
// crates/jio-core/src/block.rs
use serde::{Serialize, Deserialize};
use crate::hash::Hash;
use num_bigint::BigUint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub ghostdag_data: GhostDagData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub parents: Vec<Hash>,  // Multiple parents for DAG
    pub hash_merkle_root: Hash,
    pub accepted_id_merkle_root: Hash,
    pub utxo_commitment: Hash,
    pub timestamp: u64,
    pub bits: u32,  // Difficulty target
    pub nonce: u64,
    pub daa_score: u64,  // DAG-Aware Absolute Score
    pub blue_work: BigUint,
    pub blue_score: u64,
    pub pruning_point: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostDagData {
    pub blue_score: u64,
    pub blue_work: BigUint,
    pub selected_parent: Hash,
    pub merge_set_blues: Vec<Hash>,
    pub merge_set_reds: Vec<Hash>,
    pub blues_anticone_sizes: HashMap<Hash, u64>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self {
            header,
            transactions,
            ghostdag_data: GhostDagData::default(),
        }
    }
    
    pub fn hash(&self) -> Hash {
        self.header.hash()
    }
    
    pub fn validate(&self) -> Result<(), BlockError> {
        self.validate_header()?;
        self.validate_transactions()?;
        self.validate_ghostdag()?;
        Ok(())
    }
}
```

### Transaction Structure
```rust
// crates/jio-core/src/transaction.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub lock_time: u64,
    pub subnetwork_id: SubnetworkId,
    pub gas: u64,
    pub payload: Vec<u8>,
    
    // Cached values
    #[serde(skip)]
    pub id: Option<TransactionId>,
    #[serde(skip)]
    pub mass: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_outpoint: Outpoint,
    pub signature_script: Vec<u8>,
    pub sequence: u64,
    pub sig_op_count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub value: u64,
    pub script_pubkey: ScriptPublicKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outpoint {
    pub transaction_id: TransactionId,
    pub index: u32,
}

impl Transaction {
    pub fn calculate_hash(&self) -> TransactionId {
        // Implementation using blake3
        let mut hasher = blake3::Hasher::new();
        hasher.update(&bincode::serialize(self).unwrap());
        TransactionId(hasher.finalize().into())
    }
    
    pub fn calculate_mass(&self) -> u64 {
        // Mass calculation for fee purposes
        let base_mass = 1000;
        let input_mass = self.inputs.len() as u64 * 100;
        let output_mass = self.outputs.len() as u64 * 50;
        let payload_mass = self.payload.len() as u64;
        
        base_mass + input_mass + output_mass + payload_mass
    }
}
```

### UTXO Management
```rust
// crates/jio-core/src/utxo.rs
use dashmap::DashMap;
use parking_lot::RwLock;

pub struct UtxoSet {
    entries: DashMap<Outpoint, UtxoEntry>,
    tips: RwLock<Vec<Hash>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtxoEntry {
    pub amount: u64,
    pub script_pubkey: ScriptPublicKey,
    pub block_daa_score: u64,
    pub is_coinbase: bool,
    pub block_blue_score: u64,
}

impl UtxoSet {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
            tips: RwLock::new(Vec::new()),
        }
    }
    
    pub async fn apply_transaction(&self, tx: &Transaction, block_daa_score: u64) -> Result<(), UtxoError> {
        // Remove spent outputs
        for input in &tx.inputs {
            self.entries.remove(&input.previous_outpoint)
                .ok_or(UtxoError::MissingInput)?;
        }
        
        // Add new outputs
        for (index, output) in tx.outputs.iter().enumerate() {
            let outpoint = Outpoint {
                transaction_id: tx.id.unwrap(),
                index: index as u32,
            };
            
            let entry = UtxoEntry {
                amount: output.value,
                script_pubkey: output.script_pubkey.clone(),
                block_daa_score,
                is_coinbase: false,
                block_blue_score: 0,
            };
            
            self.entries.insert(outpoint, entry);
        }
        
        Ok(())
    }
}
```

---

## 3. Consensus Module (GhostDAG/PHANTOM)

### GhostDAG Implementation
```rust
// crates/jio-consensus/src/ghostdag.rs
use std::collections::{HashSet, HashMap, VecDeque};
use parking_lot::RwLock;
use rayon::prelude::*;

pub struct GhostDag {
    k: usize,  // PHANTOM parameter
    block_relations: DashMap<Hash, BlockRelations>,
    blue_scores: DashMap<Hash, u64>,
}

#[derive(Debug, Clone)]
pub struct BlockRelations {
    pub parents: Vec<Hash>,
    pub children: RwLock<Vec<Hash>>,
    pub is_blue: bool,
    pub blue_score: u64,
    pub selected_parent: Option<Hash>,
    pub merge_set_blues: Vec<Hash>,
    pub merge_set_reds: Vec<Hash>,
}

impl GhostDag {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            block_relations: DashMap::new(),
            blue_scores: DashMap::new(),
        }
    }
    
    pub async fn add_block(&self, block: &Block) -> Result<GhostDagData, ConsensusError> {
        let parents = &block.header.parents;
        
        // Calculate blue set using PHANTOM algorithm
        let (blue_set, red_set) = self.calculate_blue_set(block, parents).await?;
        
        // Select parent with highest blue score
        let selected_parent = self.select_parent(parents).await?;
        
        // Calculate blue work
        let blue_work = self.calculate_blue_work(&blue_set).await?;
        
        // Store block relations
        let relations = BlockRelations {
            parents: parents.clone(),
            children: RwLock::new(Vec::new()),
            is_blue: blue_set.contains(&block.hash()),
            blue_score: blue_set.len() as u64,
            selected_parent: Some(selected_parent),
            merge_set_blues: blue_set.clone(),
            merge_set_reds: red_set,
        };
        
        self.block_relations.insert(block.hash(), relations);
        
        // Update children for parent blocks
        for parent in parents {
            if let Some(mut parent_relations) = self.block_relations.get_mut(parent) {
                parent_relations.children.write().push(block.hash());
            }
        }
        
        Ok(GhostDagData {
            blue_score: blue_set.len() as u64,
            blue_work,
            selected_parent,
            merge_set_blues: blue_set,
            merge_set_reds: red_set,
            blues_anticone_sizes: HashMap::new(),
        })
    }
    
    async fn calculate_blue_set(&self, block: &Block, parents: &[Hash]) -> Result<(Vec<Hash>, Vec<Hash>), ConsensusError> {
        // PHANTOM coloring algorithm
        let mut blue_set = Vec::new();
        let mut red_set = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        // Start with parents
        for parent in parents {
            queue.push_back(*parent);
        }
        
        while let Some(current) = queue.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);
            
            // Calculate anticone size
            let anticone_size = self.calculate_anticone_size(&current, &visited).await?;
            
            if anticone_size <= self.k {
                blue_set.push(current);
            } else {
                red_set.push(current);
            }
            
            // Add ancestors to queue
            if let Some(relations) = self.block_relations.get(&current) {
                for parent in &relations.parents {
                    queue.push_back(*parent);
                }
            }
        }
        
        Ok((blue_set, red_set))
    }
}
```

### Chain Selection
```rust
// crates/jio-consensus/src/chain_selection.rs
pub struct ChainSelector {
    ghostdag: Arc<GhostDag>,
    virtual_state: RwLock<VirtualState>,
}

#[derive(Debug, Clone)]
pub struct VirtualState {
    pub selected_tip: Hash,
    pub blue_score: u64,
    pub daa_score: u64,
    pub merge_set: Vec<Hash>,
}

impl ChainSelector {
    pub async fn select_tip(&self) -> Result<Hash, ConsensusError> {
        let tips = self.get_all_tips().await?;
        
        // Select tip with highest blue score
        let best_tip = tips
            .par_iter()
            .map(|tip| {
                let blue_score = self.ghostdag.blue_scores
                    .get(tip)
                    .map(|s| *s)
                    .unwrap_or(0);
                (tip, blue_score)
            })
            .max_by_key(|(_, score)| *score)
            .map(|(tip, _)| *tip)
            .ok_or(ConsensusError::NoTips)?;
        
        Ok(best_tip)
    }
    
    pub async fn update_virtual_state(&self, new_block: &Block) -> Result<(), ConsensusError> {
        let mut state = self.virtual_state.write();
        
        // Update virtual state based on new block
        state.selected_tip = new_block.hash();
        state.blue_score = new_block.ghostdag_data.blue_score;
        state.daa_score = new_block.header.daa_score;
        state.merge_set = new_block.ghostdag_data.merge_set_blues.clone();
        
        Ok(())
    }
}
```

---

## 4. Mining Module (HeavyHash Implementation)

### HeavyHash Algorithm
```rust
// crates/jio-mining/src/heavyhash.rs
use sha3::{Keccak256, Digest};
use num_bigint::BigUint;
use rayon::prelude::*;

pub struct HeavyHash {
    matrix_size: usize,
}

impl HeavyHash {
    pub fn new() -> Self {
        Self {
            matrix_size: 4,
        }
    }
    
    pub fn hash(&self, header: &BlockHeader) -> Hash {
        // Serialize header
        let header_bytes = bincode::serialize(header).unwrap();
        
        // Generate matrix from header
        let matrix = self.generate_matrix(&header_bytes);
        
        // Perform matrix multiplication
        let result_matrix = self.matrix_multiply(&matrix, &matrix);
        
        // Hash the result with Keccak
        let mut hasher = Keccak256::new();
        for row in result_matrix {
            for val in row {
                hasher.update(&val.to_le_bytes());
            }
        }
        
        Hash(hasher.finalize().into())
    }
    
    fn generate_matrix(&self, data: &[u8]) -> Vec<Vec<u64>> {
        let mut matrix = vec![vec![0u64; self.matrix_size]; self.matrix_size];
        let mut index = 0;
        
        for i in 0..self.matrix_size {
            for j in 0..self.matrix_size {
                if index + 8 <= data.len() {
                    matrix[i][j] = u64::from_le_bytes(
                        data[index..index + 8].try_into().unwrap()
                    );
                    index += 8;
                }
            }
        }
        
        matrix
    }
    
    fn matrix_multiply(&self, a: &Vec<Vec<u64>>, b: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
        let n = a.len();
        let mut result = vec![vec![0u64; n]; n];
        
        // Parallel matrix multiplication
        result.par_iter_mut().enumerate().for_each(|(i, row)| {
            for j in 0..n {
                for k in 0..n {
                    row[j] = row[j].wrapping_add(
                        a[i][k].wrapping_mul(b[k][j])
                    );
                }
            }
        });
        
        result
    }
}

pub struct Miner {
    heavyhash: HeavyHash,
    target: BigUint,
}

impl Miner {
    pub fn new(target: BigUint) -> Self {
        Self {
            heavyhash: HeavyHash::new(),
            target,
        }
    }
    
    pub async fn mine(&self, mut header: BlockHeader) -> Result<Block, MiningError> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let target = self.target.clone();
        let heavyhash = HeavyHash::new();
        
        // Spawn mining threads
        let num_threads = num_cpus::get();
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let mut header = header.clone();
                let target = target.clone();
                let heavyhash = heavyhash.clone();
                let tx = tx.clone();
                
                tokio::spawn(async move {
                    let start_nonce = thread_id as u64 * u64::MAX / num_threads as u64;
                    let end_nonce = (thread_id + 1) as u64 * u64::MAX / num_threads as u64;
                    
                    for nonce in start_nonce..end_nonce {
                        header.nonce = nonce;
                        let hash = heavyhash.hash(&header);
                        
                        if BigUint::from_bytes_le(&hash.0) < target {
                            let _ = tx.send((header, nonce));
                            return;
                        }
                    }
                })
            })
            .collect();
        
        // Wait for solution
        match rx.await {
            Ok((header, _nonce)) => {
                Ok(Block::new(header, Vec::new()))
            }
            Err(_) => Err(MiningError::NoSolutionFound),
        }
    }
}
```

---

## 5. Network Layer (P2P Implementation)

### LibP2P Network Stack
```rust
// crates/jio-network/src/p2p.rs
use libp2p::{
    NetworkBehaviour, Swarm, PeerId,
    gossipsub::{Gossipsub, GossipsubEvent, MessageAuthenticity},
    kad::{Kademlia, KademliaEvent},
    identify::{Identify, IdentifyEvent},
    ping::{Ping, PingEvent},
};
use tokio::sync::mpsc;

#[derive(NetworkBehaviour)]
pub struct JioBehaviour {
    pub gossipsub: Gossipsub,
    pub kademlia: Kademlia,
    pub identify: Identify,
    pub ping: Ping,
}

pub struct NetworkNode {
    swarm: Swarm<JioBehaviour>,
    command_rx: mpsc::Receiver<NetworkCommand>,
    event_tx: mpsc::Sender<NetworkEvent>,
}

#[derive(Debug, Clone)]
pub enum NetworkCommand {
    BroadcastBlock(Block),
    BroadcastTransaction(Transaction),
    RequestBlocks(Vec<Hash>),
    Connect(PeerId),
}

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    NewBlock(Block),
    NewTransaction(Transaction),
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
}

impl NetworkNode {
    pub async fn new(config: NetworkConfig) -> Result<Self, NetworkError> {
        // Create libp2p swarm
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        let transport = libp2p::development_transport(local_key.clone()).await?;
        
        // Configure Gossipsub
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(1))
            .validation_mode(ValidationMode::Strict)
            .build()
            .unwrap();
            
        let gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )?;
        
        // Configure Kademlia
        let kademlia = Kademlia::new(local_peer_id, MemoryStore::new(local_peer_id));
        
        let behaviour = JioBehaviour {
            gossipsub,
            kademlia,
            identify: Identify::new(identify::Config::new(
                "/jio/1.0.0".to_string(),
                local_key.public(),
            )),
            ping: Ping::default(),
        };
        
        let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, local_peer_id)
            .build();
            
        // Listen on TCP and QUIC
        swarm.listen_on("/ip4/0.0.0.0/tcp/16110".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/udp/16110/quic".parse()?)?;
        
        Ok(Self {
            swarm,
            command_rx,
            event_tx,
        })
    }
    
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                // Handle swarm events
                event = self.swarm.select_next_some() => {
                    self.handle_swarm_event(event).await;
                }
                
                // Handle commands
                Some(cmd) = self.command_rx.recv() => {
                    self.handle_command(cmd).await;
                }
            }
        }
    }
    
    async fn handle_command(&mut self, cmd: NetworkCommand) {
        match cmd {
            NetworkCommand::BroadcastBlock(block) => {
                let message = bincode::serialize(&block).unwrap();
                self.swarm.behaviour_mut().gossipsub
                    .publish(IdentTopic::new("blocks"), message);
            }
            NetworkCommand::BroadcastTransaction(tx) => {
                let message = bincode::serialize(&tx).unwrap();
                self.swarm.behaviour_mut().gossipsub
                    .publish(IdentTopic::new("transactions"), message);
            }
            NetworkCommand::RequestBlocks(hashes) => {
                // Implement block request logic
            }
            NetworkCommand::Connect(peer_id) => {
                self.swarm.dial(peer_id);
            }
        }
    }
}
```

---

## 6. Storage Layer

### RocksDB Integration
```rust
// crates/jio-storage/src/db.rs
use rocksdb::{DB, Options, ColumnFamilyDescriptor};
use std::sync::Arc;

pub struct Database {
    db: Arc<DB>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, StorageError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_max_open_files(10000);
        opts.set_use_fsync(false);
        opts.set_bytes_per_sync(8388608);
        opts.optimize_for_point_lookup(1024);
        opts.set_table_cache_num_shard_bits(6);
        opts.set_max_write_buffer_number(32);
        opts.set_write_buffer_size(536870912);
        opts.set_target_file_size_base(1073741824);
        opts.set_min_write_buffer_number_to_merge(4);
        opts.set_level_zero_stop_writes_trigger(2000);
        opts.set_level_zero_slowdown_writes_trigger(0);
        opts.set_compaction_style(rocksdb::DBCompactionStyle::Universal);
        
        let cfs = vec![
            ColumnFamilyDescriptor::new("blocks", Options::default()),
            ColumnFamilyDescriptor::new("headers", Options::default()),
            ColumnFamilyDescriptor::new("transactions", Options::default()),
            ColumnFamilyDescriptor::new("utxos", Options::default()),
            ColumnFamilyDescriptor::new("ghostdag", Options::default()),
            ColumnFamilyDescriptor::new("relations", Options::default()),
            ColumnFamilyDescriptor::new("acceptance", Options::default()),
        ];
        
        let db = DB::open_cf_descriptors(&opts, path, cfs)?;
        
        Ok(Self {
            db: Arc::new(db),
        })
    }
    
    pub async fn store_block(&self, block: &Block) -> Result<(), StorageError> {
        let cf = self.db.cf_handle("blocks")
            .ok_or(StorageError::ColumnFamilyNotFound)?;
            
        let key = block.hash().to_bytes();
        let value = bincode::serialize(&block)?;
        
        self.db.put_cf(cf, key, value)?;
        
        // Store header separately for fast access
        let header_cf = self.db.cf_handle("headers")
            .ok_or(StorageError::ColumnFamilyNotFound)?;
        let header_value = bincode::serialize(&block.header)?;
        self.db.put_cf(header_cf, key, header_value)?;
        
        Ok(())
    }
    
    pub async fn get_block(&self, hash: &Hash) -> Result<Block, StorageError> {
        let cf = self.db.cf_handle("blocks")
            .ok_or(StorageError::ColumnFamilyNotFound)?;
            
        let value = self.db.get_cf(cf, hash.to_bytes())?
            .ok_or(StorageError::BlockNotFound)?;
            
        let block: Block = bincode::deserialize(&value)?;
        Ok(block)
    }
}
```

---

## 7. RPC Server Implementation

### JSON-RPC with Tonic
```rust
// crates/jio-rpc/src/server.rs
use tonic::{transport::Server, Request, Response, Status};
use jsonrpsee::server::{RpcModule, ServerBuilder};

pub struct RpcServer {
    blockchain: Arc<Blockchain>,
    mempool: Arc<Mempool>,
}

impl RpcServer {
    pub async fn start(config: RpcConfig) -> Result<(), RpcError> {
        let server = ServerBuilder::default()
            .build(&config.listen_addr)
            .await?;
            
        let mut module = RpcModule::new(());
        
        // Register RPC methods
        module.register_async_method("getBlock", |params, ctx| async move {
            let hash: Hash = params.parse()?;
            let block = ctx.blockchain.get_block(&hash).await?;
            Ok(serde_json::to_value(block)?)
        })?;
        
        module.register_async_method("submitTransaction", |params, ctx| async move {
            let tx: Transaction = params.parse()?;
            ctx.mempool.add_transaction(tx).await?;
            Ok(serde_json::Value::Bool(true))
        })?;
        
        module.register_async_method("getBlockTemplate", |params, ctx| async move {
            let template = ctx.blockchain.create_block_template().await?;
            Ok(serde_json::to_value(template)?)
        })?;
        
        module.register_async_method("submitBlock", |params, ctx| async move {
            let block: Block = params.parse()?;
            ctx.blockchain.add_block(block).await?;
            Ok(serde_json::Value::Bool(true))
        })?;
        
        server.start(module).await
    }
}
```

---

## 8. Implementation Timeline

### Phase 1: Foundation (Weeks 1-3)
- **Week 1**: Project setup, workspace configuration
  - Initialize cargo workspace
  - Setup CI/CD with GitHub Actions
  - Configure linting and formatting
  
- **Week 2**: Core data structures
  - Implement Block, Transaction, UTXO types
  - Hash and cryptographic primitives
  - Serialization/deserialization
  
- **Week 3**: Basic storage layer
  - RocksDB integration
  - Key-value store abstractions
  - Block storage and retrieval

### Phase 2: Consensus Implementation (Weeks 4-7)
- **Week 4-5**: GhostDAG algorithm
  - Blue selection mechanism
  - Parent selection logic
  - Anticone calculation
  
- **Week 6-7**: Chain selection
  - Virtual state management
  - Tip selection algorithm
  - Reorg handling

### Phase 3: Mining System (Weeks 8-10)
- **Week 8**: HeavyHash implementation
  - Matrix operations
  - Keccak integration
  - Hash validation
  
- **Week 9**: Mining infrastructure
  - Async mining loops
  - Multi-threaded mining
  - Difficulty adjustment
  
- **Week 10**: Mining pool support
  - Stratum protocol
  - Work distribution
  - Share validation

### Phase 4: Networking (Weeks 11-14)
- **Week 11-12**: P2P implementation
  - LibP2P integration