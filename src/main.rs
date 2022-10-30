use clap::Parser;
use cli::{Args, Command};
use install::Installer;
use versions::VersionString;

mod cli;
mod install;
mod versions;

fn main() {
	let cli = Args::parse();
	let command = cli.command;

	match command {
		Command::Install { version } => {
			let version = VersionString::from_string(&version).unwrap();
			Installer::new(version).run()
		},
	};
}
