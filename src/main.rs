use clap::Parser;
use cli::{Args, Command};
use install::Installer;
use version::VersionId;

mod cli;
mod github;
mod install;
mod version;

fn main() {
	let cli = Args::parse();
	let command = cli.command;

	match command {
		Command::Install { version } => {
			let version = VersionId::from_string(&version).unwrap();
			Installer::new(version).run()
		},
	};
}
