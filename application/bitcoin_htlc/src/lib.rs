#![feature(plugin, decl_macro)]

extern crate bitcoin;
extern crate bitcoin_rpc_client;
extern crate bitcoin_support;
extern crate bitcoin_witness;
extern crate common_types;
extern crate hex;
extern crate secp256k1_support;

#[macro_use]
extern crate log;

pub mod bitcoin_htlc;

pub use bitcoin_htlc::*;
pub use common_types::secret;
