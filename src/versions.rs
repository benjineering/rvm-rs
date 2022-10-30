use std::num::ParseIntError;

pub struct Version {
	pub major: Option<u8>,
	pub minor: Option<u8>,
	pub patch: Option<u8>,
	pub label: Option<String>,
}

impl Version {
	pub fn empty() -> Version {
		Version {
			major: None,
			minor: None,
			patch: None,
			label: None,
		}
	}

	pub fn from_string(string: &String) -> Result<Version, ParseIntError> {
		let mut version_and_label = string.split('-');

		let mut version_parts = match version_and_label.next() {
			Some(x) => x.split('.'),
			None => return Ok(Version::empty())
		};

		// TODO: DRY me up
		let major = match version_parts.next() {
			Some(x) => if x.is_empty() { None } else { Some(x.parse::<u8>()?) },
			None => None
		};
		
		let minor = match version_parts.next() {
			Some(x) => Some(x.parse::<u8>()?),
			None => None
		};
		
		let patch = match version_parts.next() {
			Some(x) => Some(x.parse::<u8>()?),
			None => None
		};

		let label = match version_and_label.next() {
			Some(x) => Some(x.to_string()),
			None => None
		};

		Ok(Version { major, minor, patch, label })
	}
}

pub struct VersionString {
	pub interpreter: Option<String>,
	pub version: Option<Version>,
}

impl VersionString {
	pub fn empty() -> VersionString {
		VersionString {
			interpreter: None,
			version: None,
		}
	}

	pub fn from_string(string: &String) -> Result<VersionString, ParseIntError> {
		let mut version_and_label = string.split('-');

		let mut version_parts = match version_and_label.next() {
			Some(x) => x.split('.'),
			None => return Ok(Version::empty())
		};

		// TODO: DRY me up
		let major = match version_parts.next() {
			Some(x) => if x.is_empty() { None } else { Some(x.parse::<u8>()?) },
			None => None
		};
		
		let minor = match version_parts.next() {
			Some(x) => Some(x.parse::<u8>()?),
			None => None
		};
		
		let patch = match version_parts.next() {
			Some(x) => Some(x.parse::<u8>()?),
			None => None
		};

		let label = match version_and_label.next() {
			Some(x) => Some(x.to_string()),
			None => None
		};

		Ok(Version { major, minor, patch, label })
	}
}

#[cfg(test)]
mod version_tests {
	use super::*;

	#[test]
	fn from_string_with_all_properties() {
		let version = Version::from_string(&String::from("3.2.0-preview2")).unwrap();
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(Some(0), version.patch);
		assert_eq!(Some(String::from("preview2")), version.label);
	}

	#[test]
	fn from_string_with_major_version() {
		let version = Version::from_string(&String::from("3")).unwrap();
		assert_eq!(Some(3), version.major);
		assert_eq!(None, version.minor);
		assert_eq!(None, version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_major_and_minor_version() {
		let version = Version::from_string(&String::from("3.20")).unwrap();
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(20), version.minor);
		assert_eq!(None, version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_major_minor_and_patch() {
		let version = Version::from_string(&String::from("1.22.5")).unwrap();
		assert_eq!(Some(1), version.major);
		assert_eq!(Some(22), version.minor);
		assert_eq!(Some(5), version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_empty_string() {
		let version = Version::from_string(&String::from("")).unwrap();
		assert_eq!(None, version.major);
		assert_eq!(None, version.minor);
		assert_eq!(None, version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_hot_garbage() {
		let version = Version::from_string(&String::from(".hgvgvt.eger-vfdvfd"));
		assert!(version.is_err());
	}
}
