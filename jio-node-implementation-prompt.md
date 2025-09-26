# JIO Full Node Implementation - AI Development Prompts

## Master Prompt: Complete JIO Node Implementation

```markdown
You are implementing a production-ready full node for the JIO blockchain in Rust. The node must integrate all core components including consensus (GhostDAG), mining (HeavyHash), networking (P2P), storage (RocksDB), and API services. 

Project Structure:
crates/jio-node/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point and CLI
│   ├── node.rs              # Core node implementation
│   ├── config.rs            # Configuration management
│   ├── services/
│   │   ├── mod.rs
│   │   ├── blockchain.rs    # Blockchain service
│   │   ├── mempool.rs       # Transaction pool service
│   │   ├── miner.rs         # Mining service
│   │   ├── network.rs       # P2P network service
│   │   ├── rpc.rs          # RPC service
│   │   ├── sync.rs         # Chain synchronization
│   │   └── monitor.rs      # Monitoring service
│   ├── managers/
│   │   ├── mod.rs
│   │   ├── chain_manager.rs # Chain state management
│   │   ├── peer_manager.rs  # Peer connection management
│   │   └── event_manager.rs # Event bus implementation
│   └── utils/
│       ├── mod.rs
│       ├── metrics.rs       # Prometheus metrics
│       └── logger.rs        # Logging configuration

Core Requirements:
- Implement a modular, service-oriented architecture
- Use tokio for async runtime with proper task management
- Implement graceful shutdown for all services
- Use channels for inter-service communication
- Include comprehensive error handling and recovery
- Support multiple node modes (full, archive, light, mining)
- Implement proper resource management and cleanup

PROVIDE COMPLETE, PRODUCTION-READY IMPLEMENTATION.
```

---

## Prompt 1: Main Entry Point and CLI

```markdown
Implement the main entry point for the JIO node with comprehensive CLI interface using clap.

File: crates/jio-node/src/main.rs

Requirements:
- Parse command-line arguments and configuration files
- Initialize logging and metrics
- Start all node services
- Handle signals for graceful shutdown
- Support different node modes

```rust
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tokio::signal;
use tracing::{info, error};

#[derive(Parser)]
#[clap(name = "jio-node")]
#[clap(about = "JIO Blockchain Full Node", version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    
    /// Configuration file path
    #[clap(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    
    /// Data directory path
    #[clap(short, long, value_name = "DIR", default_value = "~/.jio")]
    datadir: PathBuf,
    
    /// Network to connect to
    #[clap(short, long, default_value = "mainnet")]
    network: String,
    
    /// Logging level
    #[clap(short, long, default_value = "info")]
    log_level: String,
    
    /// Enable mining
    #[clap(long)]
    mine: bool,
    
    /// Mining address (required if mining is enabled)
    #[clap(long, value_name = "ADDRESS")]
    miner_address: Option<String>,
    
    /// Number of mining threads
    #[clap(long, default_value = "0")]
    mining_threads: usize,
    
    /// RPC server bind address
    #[clap(long, default_value = "127.0.0.1:16110")]
    rpc_bind: String,
    
    /// Enable RPC server
    #[clap(long)]
    rpc: bool,
    
    /// P2P listen address
    #[clap(long, default_value = "0.0.0.0:16111")]
    p2p_bind: String,
    
    /// Bootstrap nodes
    #[clap(long, value_name = "NODE")]
    addnode: Vec<String>,
    
    /// Maximum number of peers
    #[clap(long, default_value = "125")]
    max_peers: usize,
    
    /// Enable archive mode (store all blocks)
    #[clap(long)]
    archive: bool,
    
    /// Prune blocks older than this many days
    #[clap(long)]
    prune_days: Option<u32>,
    
    /// Maximum mempool size in MB
    #[clap(long, default_value = "300")]
    mempool_size: usize,
    
    /// Enable Prometheus metrics
    #[clap(long)]
    metrics: bool,
    
    /// Metrics server bind address
    #[clap(long, default_value = "127.0.0.1:9090")]
    metrics_bind: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the node
    Start,
    
    /// Initialize a new blockchain
    Init {
        /// Genesis block configuration file
        #[clap(short, long, value_name = "FILE")]
        genesis: Option<PathBuf>,
    },
    
    /// Import blocks from file
    Import {
        /// Block data file
        #[clap(value_name = "FILE")]
        file: PathBuf,
    },
    
    /// Export blocks to file
    Export {
        /// Output file
        #[clap(value_name = "FILE")]
        file: PathBuf,
        
        /// Start height
        #[clap(long)]
        from: Option<u64>,
        
        /// End height
        #[clap(long)]
        to: Option<u64>,
    },
    
    /// Validate blockchain integrity
    Validate {
        /// Perform full validation
        #[clap(long)]
        full: bool,
    },
    
    /// Database operations
    Db {
        #[clap(subcommand)]
        command: DbCommands,
    },
    
    /// Generate a new wallet
    Wallet {
        /// Output wallet file
        #[clap(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// Compact database
    Compact,
    
    /// Repair database
    Repair,
    
    /// Show database statistics
    Stats,
    
    /// Upgrade database version
    Upgrade,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    initialize_logging(&cli.log_level)?;
    
    // Load configuration
    let config = load_configuration(&cli).await?;
    
    // Execute command
    match cli.command {
        Commands::Start => {
            info!("Starting JIO node v{}", env!("CARGO_PKG_VERSION"));
            start_node(config).await?;
        }
        Commands::Init { genesis } => {
            initialize_blockchain(config, genesis).await?;
        }
        Commands::Import { file } => {
            import_blocks(config, file).await?;
        }
        Commands::Export { file, from, to } => {
            export_blocks(config, file, from, to).await?;
        }
        Commands::Validate { full } => {
            validate_blockchain(config, full).await?;
        }
        Commands::Db { command } => {
            handle_db_command(config, command).await?;
        }
        Commands::Wallet { output } => {
            generate_wallet(output).await?;
        }
    }
    
    Ok(())
}

async fn start_node(config: NodeConfig) -> Result<()> {
    // Create node instance
    let node = JioNode::new(config).await?;
    
    // Start node
    node.start().await?;
    
    // Wait for shutdown signal
    shutdown_signal().await;
    
    info!("Shutting down node...");
    node.shutdown().await?;
    
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Received shutdown signal");
}

fn initialize_logging(level: &str) -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("jio={},tower_http=debug", level).into());
    
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    Ok(())
}
```

Include implementation for all command handlers and configuration loading.
```

---

## Prompt 2: Core Node Implementation

```markdown
Implement the core JioNode struct that orchestrates all blockchain services.

File: crates/jio-node/src/node.rs

```rust
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc, broadcast, Mutex};
use tokio::task::JoinSet;
use anyhow::Result;

pub struct JioNode {
    // Core components
    config: Arc<NodeConfig>,
    blockchain: Arc<BlockchainService>,
    mempool: Arc<MempoolService>,
    network: Arc<NetworkService>,
    miner: Option<Arc<MiningService>>,
    rpc: Option<Arc<RpcService>>,
    sync: Arc<SyncService>,
    monitor: Arc<MonitoringService>,
    
    // State management
    state: Arc<RwLock<NodeState>>,
    event_bus: Arc<EventBus>,
    
    // Task management
    tasks: Mutex<JoinSet<Result<()>>>,
    shutdown_tx: broadcast::Sender<()>,
}

#[derive(Debug, Clone)]
pub enum NodeState {
    Initializing,
    Syncing(SyncProgress),
    Synchronized,
    Mining,
    Shutting,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct SyncProgress {
    pub current_height: u64,
    pub target_height: u64,
    pub peers: usize,
    pub download_rate: f64,
}

impl JioNode {
    pub async fn new(config: NodeConfig) -> Result<Self> {
        info!("Initializing JIO node with config: {:?}", config.network);
        
        // Initialize database
        let db = Arc::new(Database::new(&config.data_dir)?);
        
        // Initialize event bus
        let event_bus = Arc::new(EventBus::new());
        
        // Create shutdown channel
        let (shutdown_tx, _) = broadcast::channel(1);
        
        // Initialize blockchain service
        let blockchain = Arc::new(
            BlockchainService::new(
                db.clone(),
                config.consensus.clone(),
                event_bus.clone(),
            ).await?
        );
        
        // Initialize mempool
        let mempool = Arc::new(
            MempoolService::new(
                config.mempool.clone(),
                blockchain.clone(),
                event_bus.clone(),
            ).await?
        );
        
        // Initialize network service
        let network = Arc::new(
            NetworkService::new(
                config.network.clone(),
                blockchain.clone(),
                mempool.clone(),
                event_bus.clone(),
            ).await?
        );
        
        // Initialize sync service
        let sync = Arc::new(
            SyncService::new(
                blockchain.clone(),
                network.clone(),
                event_bus.clone(),
            ).await?
        );
        
        // Initialize mining service if enabled
        let miner = if config.mining.enabled {
            Some(Arc::new(
                MiningService::new(
                    config.mining.clone(),
                    blockchain.clone(),
                    mempool.clone(),
                    event_bus.clone(),
                ).await?
            ))
        } else {
            None
        };
        
        // Initialize RPC service if enabled
        let rpc = if config.rpc.enabled {
            Some(Arc::new(
                RpcService::new(
                    config.rpc.clone(),
                    blockchain.clone(),
                    mempool.clone(),
                    network.clone(),
                    miner.clone(),
                ).await?
            ))
        } else {
            None
        };
        
        // Initialize monitoring service
        let monitor = Arc::new(
            MonitoringService::new(
                config.monitoring.clone(),
                blockchain.clone(),
                mempool.clone(),
                network.clone(),
            ).await?
        );
        
        Ok(Self {
            config: Arc::new(config),
            blockchain,
            mempool,
            network,
            miner,
            rpc,
            sync,
            monitor,
            state: Arc::new(RwLock::new(NodeState::Initializing)),
            event_bus,
            tasks: Mutex::new(JoinSet::new()),
            shutdown_tx,
        })
    }
    
    pub async fn start(&self) -> Result<()> {
        info!("Starting JIO node services...");
        
        // Update state
        *self.state.write().await = NodeState::Syncing(SyncProgress::default());
        
        // Start services in order
        self.start_blockchain_service().await?;
        self.start_network_service().await?;
        self.start_mempool_service().await?;
        self.start_sync_service().await?;
        
        if let Some(miner) = &self.miner {
            self.start_mining_service(miner.clone()).await?;
        }
        
        if let Some(rpc) = &self.rpc {
            self.start_rpc_service(rpc.clone()).await?;
        }
        
        self.start_monitoring_service().await?;
        
        // Start event processing
        self.start_event_processor().await?;
        
        info!("JIO node started successfully");
        Ok(())
    }
    
    async fn start_blockchain_service(&self) -> Result<()> {
        let blockchain = self.blockchain.clone();
        let shutdown = self.shutdown_tx.subscribe();
        
        self.tasks.lock().await.spawn(async move {
            blockchain.run(shutdown).await
        });
        
        Ok(())
    }
    
    async fn start_network_service(&self) -> Result<()> {
        let network = self.network.clone();
        let shutdown = self.shutdown_tx.subscribe();
        
        self.tasks.lock().await.spawn(async move {
            network.run(shutdown).await
        });
        
        // Connect to bootstrap nodes
        for peer in &self.config.network.bootstrap_nodes {
            self.network.connect_peer(peer).await?;
        }
        
        Ok(())
    }
    
    async fn start_event_processor(&self) -> Result<()> {
        let mut event_rx = self.event_bus.subscribe();
        let state = self.state.clone();
        let blockchain = self.blockchain.clone();
        let mempool = self.mempool.clone();
        let network = self.network.clone();
        
        self.tasks.lock().await.spawn(async move {
            while let Ok(event) = event_rx.recv().await {
                match event {
                    Event::NewBlock(block) => {
                        // Handle new block
                        if let Err(e) = blockchain.process_block(block).await {
                            error!("Failed to process block: {}", e);
                        }
                    }
                    Event::NewTransaction(tx) => {
                        // Add to mempool
                        if let Err(e) = mempool.add_transaction(tx).await {
                            debug!("Failed to add transaction to mempool: {}", e);
                        }
                    }
                    Event::PeerConnected(peer_id) => {
                        info!("Peer connected: {}", peer_id);
                    }
                    Event::PeerDisconnected(peer_id) => {
                        info!("Peer disconnected: {}", peer_id);
                    }
                    Event::SyncCompleted => {
                        info!("Blockchain synchronization completed");
                        *state.write().await = NodeState::Synchronized;
                    }
                    _ => {}
                }
            }
            Ok(())
        });
        
        Ok(())
    }
    
    pub async fn shutdown(&self) -> Result<()> {
        info!("Initiating node shutdown...");
        
        // Update state
        *self.state.write().await = NodeState::Shutting;
        
        // Send shutdown signal
        let _ = self.shutdown_tx.send(());
        
        // Wait for all tasks to complete
        let mut tasks = self.tasks.lock().await;
        while let Some(result) = tasks.join_next().await {
            if let Err(e) = result {
                error!("Task failed during shutdown: {:?}", e);
            }
        }
        
        // Cleanup resources
        self.blockchain.shutdown().await?;
        self.mempool.shutdown().await?;
        self.network.shutdown().await?;
        
        if let Some(miner) = &self.miner {
            miner.shutdown().await?;
        }
        
        *self.state.write().await = NodeState::Stopped;
        info!("Node shutdown complete");
        
        Ok(())
    }
    
    pub async fn get_state(&self) -> NodeState {
        self.state.read().await.clone()
    }
    
    pub async fn get_info(&self) -> NodeInfo {
        NodeInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            network: self.config.network.network_id.clone(),
            state: self.get_state().await,
            height: self.blockchain.get_height().await,
            peers: self.network.get_peer_count().await,
            mempool_size: self.mempool.get_size().await,
            hashrate: self.miner.as_ref()
                .map(|m| m.get_hashrate())
                .unwrap_or(0.0),
        }
    }
}
```

Implement all service start methods and event handling logic.
```

---

## Prompt 3: Blockchain Service Implementation

```markdown
Implement the core blockchain service that manages chain state, block processing, and validation.

File: crates/jio-node/src/services/blockchain.rs

```rust
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc, broadcast, Mutex};
use dashmap::DashMap;

pub struct BlockchainService {
    // Core components
    db: Arc<Database>,
    consensus: Arc<GhostDagConsensus>,
    utxo_set: Arc<UtxoSet>,
    
    // Chain state
    chain_state: Arc<RwLock<ChainState>>,
    block_index: Arc<DashMap<Hash, BlockIndexEntry>>,
    orphan_blocks: Arc<DashMap<Hash, Block>>,
    
    // Processing queues
    block_queue: Arc<Mutex<mpsc::UnboundedSender<Block>>>,
    validation_queue: Arc<Mutex<mpsc::UnboundedSender<Block>>>,
    
    // Event bus
    event_bus: Arc<EventBus>,
    
    // Metrics
    metrics: BlockchainMetrics,
}

#[derive(Debug, Clone)]
pub struct ChainState {
    pub tip: Hash,
    pub height: u64,
    pub total_difficulty: BigUint,
    pub total_transactions: u64,
    pub chain_work: BigUint,
    pub median_time: u64,
    pub virtual_daa_score: u64,
    pub virtual_blue_score: u64,
}

#[derive(Debug, Clone)]
pub struct BlockIndexEntry {
    pub hash: Hash,
    pub height: u64,
    pub header: BlockHeader,
    pub status: BlockStatus,
    pub blue_score: u64,
    pub daa_score: u64,
    pub merge_set: Vec<Hash>,
    pub selected_parent: Hash,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockStatus {
    Invalid,
    Orphan,
    HeaderOnly,
    Valid,
    FullyValidated,
    InMainChain,
}

impl BlockchainService {
    pub async fn new(
        db: Arc<Database>,
        config: ConsensusConfig,
        event_bus: Arc<EventBus>,
    ) -> Result<Self> {
        // Initialize consensus
        let consensus = Arc::new(GhostDagConsensus::new(config));
        
        // Load UTXO set
        let utxo_set = Arc::new(UtxoSet::load_from_db(&db).await?);
        
        // Load chain state
        let chain_state = Arc::new(RwLock::new(
            ChainState::load_from_db(&db).await?
        ));
        
        // Create processing queues
        let (block_tx, block_rx) = mpsc::unbounded_channel();
        let (validation_tx, validation_rx) = mpsc::unbounded_channel();
        
        let service = Self {
            db,
            consensus,
            utxo_set,
            chain_state,
            block_index: Arc::new(DashMap::new()),
            orphan_blocks: Arc::new(DashMap::new()),
            block_queue: Arc::new(Mutex::new(block_tx)),
            validation_queue: Arc::new(Mutex::new(validation_tx)),
            event_bus,
            metrics: BlockchainMetrics::new(),
        };
        
        // Load block index
        service.load_block_index().await?;
        
        // Start processing tasks
        service.start_block_processor(block_rx).await;
        service.start_validation_processor(validation_rx).await;
        
        Ok(service)
    }
    
    pub async fn process_block(&self, block: Block) -> Result<()> {
        let block_hash = block.hash();
        
        // Quick checks
        if self.block_index.contains_key(&block_hash) {
            return Ok(()); // Already have this block
        }
        
        // Add to processing queue
        self.block_queue.lock().await.send(block)?;
        
        Ok(())
    }
    
    async fn start_block_processor(
        &self,
        mut rx: mpsc::UnboundedReceiver<Block>
    ) {
        let consensus = self.consensus.clone();
        let utxo_set = self.utxo_set.clone();
        let chain_state = self.chain_state.clone();
        let block_index = self.block_index.clone();
        let orphan_blocks = self.orphan_blocks.clone();
        let validation_queue = self.validation_queue.clone();
        let event_bus = self.event_bus.clone();
        let metrics = self.metrics.clone();
        
        tokio::spawn(async move {
            while let Some(block) = rx.recv().await {
                let timer = metrics.block_processing_time.start_timer();
                
                // Validate block header
                if let Err(e) = consensus.validate_header(&block.header).await {
                    error!("Invalid block header: {}", e);
                    metrics.invalid_blocks.inc();
                    continue;
                }
                
                // Check if we have all parent blocks
                let parents = &block.header.parents;
                let mut missing_parents = Vec::new();
                
                for parent_hash in parents {
                    if !block_index.contains_key(parent_hash) {
                        missing_parents.push(*parent_hash);
                    }
                }
                
                if !missing_parents.is_empty() {
                    // This is an orphan block
                    info!("Orphan block {}, missing parents: {:?}", 
                          block.hash(), missing_parents);
                    orphan_blocks.insert(block.hash(), block);
                    metrics.orphan_blocks.inc();
                    
                    // Request missing parents
                    event_bus.publish(Event::RequestBlocks(missing_parents)).await;
                    continue;
                }
                
                // Process block with GhostDAG
                match consensus.process_block(&block, &block_index).await {
                    Ok(ghostdag_data) => {
                        // Update block index
                        let entry = BlockIndexEntry {
                            hash: block.hash(),
                            height: block.header.daa_score, // Use DAA score as height
                            header: block.header.clone(),
                            status: BlockStatus::Valid,
                            blue_score: ghostdag_data.blue_score,
                            daa_score: block.header.daa_score,
                            merge_set: ghostdag_data.merge_set_blues.clone(),
                            selected_parent: ghostdag_data.selected_parent,
                        };
                        
                        block_index.insert(block.hash(), entry);
                        
                        // Send for full validation
                        validation_queue.lock().await.send(block).await.ok();
                    }
                    Err(e) => {
                        error!("Failed to process block with GhostDAG: {}", e);
                        metrics.invalid_blocks.inc();
                    }
                }
                
                timer.observe_duration();
            }
        });
    }
    
    async fn start_validation_processor(
        &self,
        mut rx: mpsc::UnboundedReceiver<Block>
    ) {
        let consensus = self.consensus.clone();
        let utxo_set = self.utxo_set.clone();
        let chain_state = self.chain_state.clone();
        let block_index = self.block_index.clone();
        let db = self.db.clone();
        let event_bus = self.event_bus.clone();
        let metrics = self.metrics.clone();
        
        tokio::spawn(async move {
            while let Some(block) = rx.recv().await {
                let timer = metrics.validation_time.start_timer();
                
                // Full block validation
                match validate_block_transactions(&block, &utxo_set).await {
                    Ok(()) => {
                        // Apply block to UTXO set
                        if let Err(e) = apply_block_to_utxo(&block, &utxo_set).await {
                            error!("Failed to apply block to UTXO set: {}", e);
                            continue;
                        }
                        
                        // Update chain state if this is the new tip
                        let mut state = chain_state.write().await;
                        if should_update_tip(&block, &state, &consensus).await {
                            state.tip = block.hash();
                            state.height = block.header.daa_score;
                            state.virtual_blue_score = block.header.blue_score;
                            state.virtual_daa_score = block.header.daa_score;
                            
                            // Store block
                            db.store_block(&block).await.ok();
                            
                            // Publish new block event
                            event_bus.publish(Event::NewBlock(block.clone())).await;
                            
                            info!("New tip: {} at height {}", 
                                  block.hash(), state.height);
                        }
                        
                        // Update block status
                        if let Some(mut entry) = block_index.get_mut(&block.hash()) {
                            entry.status = BlockStatus::FullyValidated;
                        }
                        
                        metrics.validated_blocks.inc();
                        
                        // Check if any orphans can now be processed
                        check_orphans(&block, &orphan_blocks, &block_index).await;
                    }
                    Err(e) => {
                        error!("Block validation failed: {}", e);
                        
                        // Mark block as invalid
                        if let Some(mut entry) = block_index.get_mut(&block.hash()) {
                            entry.status = BlockStatus::Invalid;
                        }
                        
                        metrics.invalid_blocks.inc();
                    }
                }
                
                timer.observe_duration();
            }
        });
    }
    
    async fn validate_block_transactions(
        block: &Block,
        utxo_set: &UtxoSet
    ) -> Result<()> {
        // Validate all transactions
        for (i, tx) in block.transactions.iter().enumerate() {
            // Skip coinbase
            if i == 0 {
                validate_coinbase_transaction(tx, block)?;
                continue;
            }
            
            // Check inputs exist in UTXO set
            for input in &tx.inputs {
                if !utxo_set.contains(&input.previous_outpoint).await {
                    return Err(anyhow!("Missing input UTXO"));
                }
            }
            
            // Validate scripts
            validate_transaction_scripts(tx, utxo_set).await?;
            
            // Check no double spends within block
            check_double_spend(tx, &block.transactions[..i])?;
        }
        
        Ok(())
    }
    
    pub async fn get_block(&self, hash: &Hash) -> Result<Option<Block>> {
        // Check memory first
        if let Some(entry) = self.block_index.get(hash) {
            if entry.status == BlockStatus::FullyValidated {
                // Load from database
                return self.db.get_block(hash).await;
            }
        }
        Ok(None)
    }
    
    pub async fn get_height(&self) -> u64 {
        self.chain_state.read().await.height
    }
    
    pub async fn create_block_template(
        &self,
        miner_address: &Address
    ) -> Result<BlockTemplate> {
        let state = self.chain_state.read().await;
        
        // Select parent blocks using GhostDAG
        let parents = self.consensus.select_parents(&state.tip).await?;
        
        // Get transactions from mempool
        let transactions = self.select_transactions_for_block().await?;
        
        // Calculate fees
        let total_fees: u64 = transactions.iter()
            .map(|tx| calculate_transaction_fee(tx))
            .sum();
        
        // Create coinbase transaction
        let coinbase = create_coinbase_transaction(
            miner_address,
            state.height + 1,
            total_fees
        )?;
        
        // Build block template
        let template = BlockTemplate {