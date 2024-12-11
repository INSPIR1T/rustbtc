use crate::U256;
use uuid::Uuid;
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
pub struct Block {
    pub header: BlockHeader,
    pub transaction: Vec<Transaction>,
}
pub struct BlockHeader {
    /// timestamp of the block
    pub timestamp: u64,
    /// nonce used to mine the block
    pub nonce: u64,
    /// hash of the previous block
    pub previous_block_hash: [u8; 32],
    /// merkle root of the block's transactions
    pub merkle_root: [u8; 32],
    /// target
    pub target: U256,
}
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}
pub struct TransactionInput {
    pub prev_transaction_hash: [u8; 32],
    pub signature: [u8; 64],
}
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: [u8; 32],
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl Block {
    pub fn new(header: BlockHeader, transaction: Vec<Transaction>) -> Self {
        Self {
            header,
            transaction,
        }
    }

    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}

impl BlockHeader {
    pub fn new(
        timestamp: u64,
        nonce: u64,
        previous_block_hash: [u8; 32],
        merkle_root: [u8; 32],
        target: U256,
    ) -> Self {
        Self {
            timestamp,
            nonce,
            previous_block_hash,
            merkle_root,
            target,
        }
    }

    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}
