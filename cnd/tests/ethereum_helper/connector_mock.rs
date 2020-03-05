use anyhow::Context;
use async_trait::async_trait;
use cnd::{
    btsieve::{ethereum::ReceiptByHash, BlockByHash, LatestBlock},
    ethereum::{Block, TransactionReceipt, H256},
};
use futures::{future::IntoFuture, Future};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct EthereumConnectorMock {
    all_blocks: HashMap<H256, Block>,
    latest_blocks: Vec<Block>,
    receipts: HashMap<H256, TransactionReceipt>,
    latest_time_return_block: Instant,
    current_latest_block_index: usize,
}

impl EthereumConnectorMock {
    pub fn new(
        latest_blocks: impl IntoIterator<Item = Block>,
        all_blocks: impl IntoIterator<Item = Block>,
        receipts: Vec<(H256, TransactionReceipt)>,
    ) -> Self {
        let all_blocks = all_blocks
            .into_iter()
            .fold(HashMap::new(), |mut hm, block| {
                hm.insert(block.hash.unwrap(), block);
                hm
            });

        let latest_blocks = latest_blocks.into_iter().collect();

        EthereumConnectorMock {
            all_blocks,
            latest_blocks,
            latest_time_return_block: Instant::now(),
            current_latest_block_index: 0,
            receipts: receipts.into_iter().collect(),
        }
    }
}

#[async_trait]
impl LatestBlock for EthereumConnectorMock {
    type Block = Block;
    type BlockHash = H256;

    async fn latest_block(&mut self) -> anyhow::Result<Self::Block> {
        if self.latest_blocks.is_empty() {
            return Err(anyhow::Error::from(Error::NoMoreBlocks));
        }

        let latest_block = self.latest_blocks[self.current_latest_block_index].clone();
        if self.latest_time_return_block.elapsed() >= Duration::from_secs(1) {
            self.latest_time_return_block = Instant::now();
            if self
                .latest_blocks
                .get(self.current_latest_block_index + 1)
                .is_some()
            {
                self.current_latest_block_index += 1;
            }
        }
        Ok(latest_block)
    }
}

impl BlockByHash for EthereumConnectorMock {
    type Block = Block;
    type BlockHash = H256;

    fn block_by_hash(
        &self,
        block_hash: Self::BlockHash,
    ) -> Box<dyn Future<Item = Self::Block, Error = anyhow::Error> + Send + 'static> {
        Box::new(
            self.all_blocks
                .get(&block_hash)
                .cloned()
                .ok_or_else(|| anyhow::Error::from(Error::UnknownHash(block_hash)))
                .into_future(),
        )
    }
}

#[async_trait]
impl ReceiptByHash for EthereumConnectorMock {
    async fn receipt_by_hash(&self, transaction_hash: H256) -> anyhow::Result<TransactionReceipt> {
        self.receipts
            .get(&transaction_hash)
            .cloned()
            .with_context(|| format!("could not find block with hash {}", transaction_hash))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ran out of blocks in chain")]
    NoMoreBlocks,
    #[error("could not find block with hash {0}")]
    UnknownHash(H256),
}
