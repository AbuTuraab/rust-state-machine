mod balances;
mod system;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<types::AccountId, 
		types::BlockNumber,
		types::Nonce>,
	balances: balances::Pallet<types::AccountId, 
	types::Balance>,
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), 
			balances: balances::Pallet::new() }
	}
}

fn main() {
	// println!("Hello, world!");

	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	// let  bob = "bob".to_string();
	// let charlie = "charlie".to_string();

	runtime.balances.set_balance(&alice, 100);

	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// 	 let inc_nonce = runtime.system.inc_nonce(&alice);

	// 	let bob_transfer = runtime.balances.transfer(alice.clone(), bob, 30).map_err(|e | eprint!("{}", e));

	// let another_inc_nonce = runtime.system.inc_nonce(&alice);
	// let charlie_transfer = runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{}", e));

	println!("{:#?}", runtime)
}

// #[cfg(test)]
// mod test {
//     use crate::Runtime;

//     #[test]
//     fn init_system() {
//         let mut runtime = Runtime::new();

//     runtime.system.inc_block_number();
//      assert_eq!(runtime.system.block_number(), 1);

//     }
// }
