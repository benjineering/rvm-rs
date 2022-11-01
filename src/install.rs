use crate::version::Version;
use crate::ruby_version::RubyVersion;
use crate::github::get_available_versions;

pub async fn install(version: Version) {
	let ruby_versions = get_available_versions().await.unwrap();

	let matched_version = RubyVersion::find(version, &ruby_versions);

	if matched_version.is_some() {
		print!("{}", matched_version.unwrap().version);
	}

}
