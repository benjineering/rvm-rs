use crate::version::Version;
use crate::ruby_version::RubyVersion;
use std::collections::HashMap;
use octocrab::models::repos::Release;

pub async fn get_available_versions() -> Result<Vec<RubyVersion>, ()> {
	let releases = get_all_releases().await.or_else(|_| Err(()))?;
	let mut versions: HashMap<String, RubyVersion> = HashMap::new();

	for release in releases {
		for asset in release.assets {
			if !asset.name.ends_with(".7z") {
				continue;
			}

			let id = Version::from_github_asset_name(&asset.name);

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

	let mut results: Vec<RubyVersion> = versions.into_values().collect();
	results.sort_unstable();
	Ok(results)
}

async fn get_all_releases() -> Result<Vec<Release>, ()> {
	let octocrab = octocrab::instance();

	let page = octocrab
		.repos("oneclick", "rubyinstaller2")
		.releases()
		.list()
		.send()
		.await
		.or_else(|_| Err(()))?;

	octocrab.all_pages::<Release>(page).await.or_else(|_| Err(()))
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
