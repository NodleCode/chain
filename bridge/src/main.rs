extern crate clap; 
#[macro_use] extern crate nickel;

extern crate substrate_subxt;
extern crate futures;
extern crate sp_keyring;
extern crate sp_core;

use clap::{Arg, App, SubCommand};
use nickel::{Nickel, HttpRouter};

use futures::future::Future;
use sp_keyring::sr25519::sr25519::Pair as SrPair;
use sp_core::crypto::Pair;
use substrate_subxt::{
	balances,
    system::System,
    DefaultNodeRuntime as Runtime,
};

fn main() {
	let matches = App::new("nodle-chain-bridge")
		.version("1.0")
		.author("Eliott Teissonniere <git.eliott@teissonniere.org>")
		.about("Expose an easy to use REST API to work with the Nodle Chain")
		.arg(Arg::with_name("secret-key")
			.short("k")
			.long("key")
			.value_name("SECRET_KEY")
			.help("Set secret key to use to send transactions (sr25519)")
			.takes_value(true))
		.arg(Arg::with_name("host")
			.short("h")
			.long("host")
			.value_name("HOST")
			.help("Host where the server is listening")
			.takes_value(true)
			.default_value("127.0.0.1:8080"))
		.arg(Arg::with_name("substrate_url")
			.short("u")
			.long("url")
			.value_name("SUBSTRATE_URL")
			.help("Substrate rpc url")
			.takes_value(true)
			.default_value("ws://127.0.0.1:9944"))
		.get_matches();

	let maybe_bridge_seed = matches.value_of("secret-key");
	let mut server = Nickel::new();

	// If a secret key was configured, enable oracle paths
	if maybe_bridge_seed.is_some() {
		let bridge_signer = SrPair::from_string(maybe_bridge_seed.unwrap(), None)
			.expect("couldn't load secret key");
		println!("Bridge is using address {}", bridge_signer.public());
		//server.post("/oracle/reward")
	}

	// TX builder helpers
	//server.post("/tx/transfer")

	// TX submission helper
	//server.post("/tx")

	// Exporer helpers
	//server.get("/chain/account/:address")

	server.listen(matches.value_of("host").unwrap());
}
