extern crate clap; 
#[macro_use] extern crate nickel;

use clap::{Arg, App, SubCommand};
use nickel::{Nickel, HttpRouter};

fn main() {
	let matches = App::new("nodle-chain-bridge")
		.version("1.0")
		.author("Eliott Teissonniere <git.eliott@teissonniere.org>")
		.about("Expose an easy to use REST API to work with the Nodle Chain")
		.arg(Arg::with_name("secret-key")
			.short("k")
			.long("key")
			.value_name("SECRET_KEY")
			.help("Set secret key to use to send transactions")
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

	let mut server = Nickel::new();

	// If a secret key was configured, enable oracle paths
	//server.post("/oracle/reward")

	// TX builder helpers
	//server.post("/tx/transfer")

	// TX submission helper
	//server.post("/tx")

	// Exporer helpers
	//server.get("/chain/account/:address")

	server.listen(matches.value_of("host").unwrap());
}
