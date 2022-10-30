use clap::Parser;

mod cli;
mod install;
mod versions;

fn main() {
	let cli = cli::Args::parse();
	let command = cli.command;
}
