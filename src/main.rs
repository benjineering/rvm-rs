use clap::Parser;

use cli::{Args, Command};
use version::Version;
use install::install;
use list::list_versions;

mod cli;
mod github;
mod install;
mod list;
mod ruby_version;
mod version;

#[tokio::main]
async fn main() {
	let cli = Args::parse();
	let command = cli.command;

	match command {
		Command::List => list_versions().await,
		Command::Install { version } => {
			let version = Version::from_string(&version).unwrap();
			install(version).await;
		},
	};
}
