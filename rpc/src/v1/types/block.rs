// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use serde::{Serialize, Serializer};
use util::numbers::*;
use v1::types::{Bytes, Transaction, OptionalValue};

/// Block Transactions
#[derive(Debug)]
pub enum BlockTransactions {
	/// Only hashes
	Hashes(Vec<H256>),
	/// Full transactions
	Full(Vec<Transaction>)
}

impl Serialize for BlockTransactions {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer {
		match *self {
			BlockTransactions::Hashes(ref hashes) => hashes.serialize(serializer),
			BlockTransactions::Full(ref ts) => ts.serialize(serializer)
		}
	}
}

/// Block representation
#[derive(Debug, Serialize)]
pub struct Block {
	/// Hash of the block
	pub hash: OptionalValue<H256>,
	/// Hash of the parent
	#[serde(rename="parentHash")]
	pub parent_hash: H256,
	/// Hash of the uncles
	#[serde(rename="sha3Uncles")]
	pub uncles_hash: H256,
	/// Authors address
	pub author: Address,
	// TODO: get rid of this one
	/// ?
	pub miner: Address,
	/// State root hash
	#[serde(rename="stateRoot")]
	pub state_root: H256,
	/// Transactions root hash
	#[serde(rename="transactionsRoot")]
	pub transactions_root: H256,
	/// Transactions receipts root hash
	#[serde(rename="receiptsRoot")]
	pub receipts_root: H256,
	/// Block number
	pub number: OptionalValue<U256>,
	/// Gas Used
	#[serde(rename="gasUsed")]
	pub gas_used: U256,
	/// Gas Limit
	#[serde(rename="gasLimit")]
	pub gas_limit: U256,
	/// Extra data
	#[serde(rename="extraData")]
	pub extra_data: Bytes,
	/// Logs bloom
	#[serde(rename="logsBloom")]
	pub logs_bloom: H2048,
	/// Timestamp
	pub timestamp: U256,
	/// Difficulty
	pub difficulty: U256,
	/// Total difficulty
	#[serde(rename="totalDifficulty")]
	pub total_difficulty: U256,
	/// Seal fields
	#[serde(rename="sealFields")]
	pub seal_fields: Vec<Bytes>,
	/// Uncles' hashes
	pub uncles: Vec<H256>,
	/// Transactions
	pub transactions: BlockTransactions
}

#[cfg(test)]
mod tests {
	use serde_json;
	use util::numbers::*;
	use v1::types::{Transaction, Bytes, OptionalValue};
	use super::*;

	#[test]
	fn test_serialize_block_transactions() {
		let t = BlockTransactions::Full(vec![Transaction::default()]);
		let serialized = serde_json::to_string(&t).unwrap();
		assert_eq!(serialized, r#"[{"hash":"0x0000000000000000000000000000000000000000000000000000000000000000","nonce":"0x00","blockHash":null,"blockNumber":null,"transactionIndex":null,"from":"0x0000000000000000000000000000000000000000","to":null,"value":"0x00","gasPrice":"0x00","gas":"0x00","input":"0x","creates":null}]"#);

		let t = BlockTransactions::Hashes(vec![H256::default()]);
		let serialized = serde_json::to_string(&t).unwrap();
		assert_eq!(serialized, r#"["0x0000000000000000000000000000000000000000000000000000000000000000"]"#);
	}

	#[test]
	fn test_serialize_block() {
		let block = Block {
			hash: OptionalValue::Value(H256::default()),
			parent_hash: H256::default(),
			uncles_hash: H256::default(),
			author: Address::default(),
			miner: Address::default(),
			state_root: H256::default(),
			transactions_root: H256::default(),
			receipts_root: H256::default(),
			number: OptionalValue::Value(U256::default()),
			gas_used: U256::default(),
			gas_limit: U256::default(),
			extra_data: Bytes::default(),
			logs_bloom: H2048::default(),
			timestamp: U256::default(),
			difficulty: U256::default(),
			total_difficulty: U256::default(),
			seal_fields: vec![Bytes::default(), Bytes::default()],
			uncles: vec![],
			transactions: BlockTransactions::Hashes(vec![])
		};

		let serialized = serde_json::to_string(&block).unwrap();
		assert_eq!(serialized, r#"{"hash":"0x0000000000000000000000000000000000000000000000000000000000000000","parentHash":"0x0000000000000000000000000000000000000000000000000000000000000000","sha3Uncles":"0x0000000000000000000000000000000000000000000000000000000000000000","author":"0x0000000000000000000000000000000000000000","miner":"0x0000000000000000000000000000000000000000","stateRoot":"0x0000000000000000000000000000000000000000000000000000000000000000","transactionsRoot":"0x0000000000000000000000000000000000000000000000000000000000000000","receiptsRoot":"0x0000000000000000000000000000000000000000000000000000000000000000","number":"0x00","gasUsed":"0x00","gasLimit":"0x00","extraData":"0x","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","timestamp":"0x00","difficulty":"0x00","totalDifficulty":"0x00","sealFields":["0x","0x"],"uncles":[],"transactions":[]}"#);
	}
}
