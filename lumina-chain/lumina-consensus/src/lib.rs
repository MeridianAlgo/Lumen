use lumina_types::block::{Block, Vote, BlockHeader};
use lumina_types::transaction::Transaction;
use lumina_types::instruction::StablecoinInstruction;
use lumina_types::state::GlobalState;
use lumina_execution::{execute_transaction, ExecutionContext};
use lumina_network::{NetworkCommand, NetworkEvent};
use tokio::sync::{mpsc, RwLock};
use std::sync::Arc;
use tracing::{info, error};

pub struct ConsensusService {
    state: Arc<RwLock<GlobalState>>,
    network_tx: mpsc::Sender<NetworkCommand>,
    tx_rx: mpsc::Receiver<Transaction>,
    mempool: Vec<Transaction>,
}

impl ConsensusService {
    pub fn new(
        state: Arc<RwLock<GlobalState>>,
        network_tx: mpsc::Sender<NetworkCommand>,
        tx_rx: mpsc::Receiver<Transaction>,
    ) -> Self {
        Self {
            state,
            network_tx,
            tx_rx,
            mempool: Vec::new(),
        }
    }

    pub async fn run(mut self) {
        info!("Starting Consensus Service (Mocked Malachite Loop)...");
        
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        let mut height = 1;
        let mut prev_hash = [0u8; 32];

        loop {
            tokio::select! {
                Some(tx) = self.tx_rx.recv() => {
                    self.mempool.push(tx);
                }
                _ = interval.tick() => {
                    // Propose Block
                    if self.mempool.is_empty() {
                        continue;
                    }

                    let txs: Vec<Transaction> = self.mempool.drain(..).collect();
                    info!("Consensus: Proposing block {} with {} txs", height, txs.len());

                    let mut state_guard = self.state.write().await;
                    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

                    // Execute Transactions
                    let mut valid_txs = Vec::new();
                    let mut ctx = ExecutionContext {
                        state: &mut state_guard,
                        height,
                        timestamp,
                    };

                    for tx in txs {
                        match execute_transaction(&tx, &mut ctx) {
                            Ok(_) => valid_txs.push(tx),
                            Err(e) => {
                                error!("Tx execution failed: {}", e);
                            }
                        }
                    }

                    if valid_txs.is_empty() {
                        continue;
                    }

                    // Create Block
                    let block = Block {
                        header: BlockHeader {
                            height,
                            prev_hash,
                            transactions_root: [0u8; 32], // TODO
                            state_root: [0u8; 32], // TODO
                            timestamp,
                            proposer: [0u8; 32],
                        },
                        transactions: valid_txs.clone(),
                        votes: Vec::new(),
                    };
                    
                    prev_hash = block.hash();
                    
                    // Broadcast
                    let block_bytes = bincode::serialize(&block).unwrap();
                    let _ = self.network_tx.send(NetworkCommand::BroadcastBlock(block_bytes)).await;

                    info!("Consensus: Committed block {} with {} valid txs. Total LUSD Supply: {}", height, block.transactions.len(), state_guard.total_lusd_supply);
                    height += 1;
                }
            }
        }
    }
}
