use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
	Install {
		version: String,
	},
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	#[command(subcommand)]
	pub command: Command,
}
