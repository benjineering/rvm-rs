use crate::version::Version;
use std::cmp;

#[derive(Eq)]
#[derive(Debug)]
pub struct RubyVersion {
	pub version: Version,
	pub url: String,
	pub publish_timestamp: i64,
}

impl RubyVersion {
	pub fn new(version: Version, url: String, publish_timestamp: i64) -> RubyVersion {
		RubyVersion { version, url: url, publish_timestamp }
	}

	pub fn find(needle: Version, haystack: &Vec<RubyVersion>) -> Option<&RubyVersion> {
		let mut found: Option<&RubyVersion> = None;

		for ruby_version in haystack {
			if needle.includes(&ruby_version.version) && (found.is_none() || ruby_version > &found.unwrap()) {
				found = Some(ruby_version);
			}
		}

		found
	}
}

impl cmp::Ord for RubyVersion {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

impl cmp::PartialOrd for RubyVersion {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.version.cmp(&other.version))
    }
}

impl cmp::PartialEq for RubyVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}
