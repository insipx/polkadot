// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Polkadot-specific RPCs implementation.

#![warn(missing_docs)]

use std::sync::Arc;

use polkadot_primitives::Block;
use node_primitives::AccountNonceApi;
use node_rpc::accounts::{Accounts, AccountsApi};
use sr_primitives::traits::ProvideRuntimeApi;
use transaction_pool::txpool::{ChainApi, Pool};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpc_core::IoHandler<substrate_rpc::Metadata>;

/// Instantiate all RPC extensions.
pub fn create<C, P>(client: Arc<C>, pool: Arc<Pool<P>>) -> RpcExtension where
	C: ProvideRuntimeApi,
	C: client::blockchain::HeaderBackend<Block>,
	C: Send + Sync + 'static,
	C::Api: AccountNonceApi<Block>,
	P: ChainApi + Sync + Send + 'static,
{
	let mut io = jsonrpc_core::IoHandler::default();
	io.extend_with(
		AccountsApi::to_delegate(Accounts::new(client.clone(), pool))
	);
	io
}
