use system::Config;

mod balances;
mod support;
mod system;

use crate::support::Dispatch;
// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {

	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
	BalancesTransfer { to: types::AccountId, amount: types::Balance },
}

/*
	TODO:
	Implement the `system::Config` trait you created on your `Runtime`.
	Use `Self` to satisfy the generic parameter required for `system::Pallet`.
*/

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,

	balances: balances::Pallet<Self>,
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}
impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err("Invalid block number");
		}
		for support::Extrinsic { caller, call } in block.extrinsics.into_iter() {
			self.system.inc_nonce(&caller);
			let res = self.dispatch(caller, call).map_err(|e| {
				eprintln!("Extrinsic Error\n\tBlock {}\n\tError: {}", block.header.block_number, e)
			});
		}

		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
			RuntimeCall::BalancesTransfer { to, amount } => {
				self.balances.transfer(caller, to, amount)?;
			},
		}
		Ok(())
	}
}



fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	runtime.balances.set_balance(&alice, 100);

	// // start emulating a block
	// runtime.system.inc_block_number();
	// assert_eq!(runtime.system.block_number(), 1);

	// // first transaction
	// runtime.system.inc_nonce(&alice);
	// let _res = runtime
	// 	.balances
	// 	.transfer(alice.clone(), bob, 30)
	// 	.map_err(|e| eprintln!("{}", e));

	// // second transaction
	// runtime.system.inc_nonce(&alice);
	// let _res = runtime.
	// balances.transfer(alice, charlie, 20)
	// .map_err(|e| eprintln!("{}", e));


	// start emulating a block
	let block_1 = types::Block {
		header: types::Header {block_number:1},

		extrinsics: vec![
			support::Extrinsic {
				caller: alice,
				call: RuntimeCall::BalancesTransfer { 
					to: bob, amount: 69 },
			},
		],
	};

	runtime.execute_block(block_1).expect("block execution failed");

	println!("{:#?}", runtime);
}
