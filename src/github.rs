use crate::version::VersionId;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RubyVersion {
	pub id: VersionId,
	pub url: String,
	pub publish_timestamp: i64,
}

impl RubyVersion {
	fn new(id: VersionId, url: String, publish_timestamp: i64) -> RubyVersion {
		RubyVersion { id, url: url, publish_timestamp }
	}
}

pub async fn get_available_versions() -> Result<Vec<RubyVersion>, ()> {
	let release_response = octocrab::instance()
		.repos("oneclick", "rubyinstaller2")
		.releases()
		.list()
		// .per_page(100)
		// .page(5u32)
		.send()
		.await
		.or_else(|_| Err(()))?;

	let mut versions: HashMap<String, RubyVersion> = HashMap::new();

	for release in release_response.items {
		for asset in release.assets {
			if !asset.name.ends_with(".7z") {
				continue;
			}

			let id = VersionId::from_github_asset_name(&asset.name);

			if id.is_err() || release.published_at.is_none() {
				continue; // TODO: log warning
			}
			
			let asset_name = asset.name.to_string();
			let timestamp = release.published_at.unwrap().timestamp();
			
			if versions.contains_key(&asset_name) {
				let existing_version = versions.get(&asset_name).unwrap();

				if timestamp > existing_version.publish_timestamp {
					let ruby_version = RubyVersion::new(id.unwrap(), asset.url.to_string(), timestamp);
					versions.insert(asset_name, ruby_version);
				}
			}
			else {
				let ruby_version = RubyVersion::new(id.unwrap(), asset.url.to_string(), timestamp);
				versions.insert(asset_name, ruby_version);
			}
		}
	}

	Ok(versions.into_values().collect())
}

// TODO: label test as long running
#[cfg(test)]
mod github_tests {
	// use super::*;

	#[tokio::test]
	async fn get_available_versions() {
		let versions = crate::github::get_available_versions().await;
		assert!(versions.is_ok());
	}
}
