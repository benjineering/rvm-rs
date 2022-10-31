use crate::version::VersionId;

#[derive(Debug)]
pub struct RubyVersion {
	id: VersionId,
	url: String
}

impl RubyVersion {
	fn new(id: VersionId, url: String) -> RubyVersion {
		RubyVersion { id, url }
	}
}

pub async fn get_available_versions() -> Result<Vec<RubyVersion>, ()> {
	println!("blorp");

	let release_response = octocrab::instance()
		.repos("oneclick", "rubyinstaller2")
		.releases()
		.list()
		// .per_page(100)
		// .page(5u32)
		.send()
		.await
		.or_else(|_| Err(()))?;

	let mut versions: Vec<RubyVersion> = vec!();

	for release in release_response.items {
		for asset in release.assets {
			if asset.name.ends_with(".7z") {
				let id = VersionId::from_string(&asset.name);

				if id.is_ok() {
					versions.push(RubyVersion::new(id.unwrap(), asset.url.to_string()));
				}
			}
		}
	}

	Ok(versions)
}

#[cfg(test)]
mod github_tests {
	// use super::*;

	#[tokio::test]
	async fn get_available_versions() {
		let versions = crate::github::get_available_versions().await;

		println!("{:?}", versions);

		assert!(versions.is_ok());
	}
}
