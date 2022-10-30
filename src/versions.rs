#[derive(PartialEq)]
#[derive(Debug)]
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

	pub fn from_string(string: &str) -> Result<Version, ()> {
		let mut version_and_label = string.split('-');

		let mut version_parts = match version_and_label.next() {
			Some(x) => x.split('.'),
			None => return Ok(Version::empty())
		};

		// TODO: DRY me up
		let major = match version_parts.next() {
			Some(x) => if x.is_empty() { None } else { Some(Self::parse_int(x)?) },
			None => None
		};
		
		let minor = match version_parts.next() {
			Some(x) => Some(Self::parse_int(x)?),
			None => None
		};
		
		let patch = match version_parts.next() {
			Some(x) => Some(Self::parse_int(x)?),
			None => None
		};

		let label = match version_and_label.next() {
			Some(x) => Some(x.to_string()),
			None => None
		};

		Ok(Version { major, minor, patch, label })
	}

	fn parse_int(string: &str) -> Result<u8, ()> {
		string.parse::<u8>().or_else(|_| Err(()))
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

	pub fn from_string(string: &str) -> Result<VersionString, ()> {
		if string.is_empty() { 
			return Ok(VersionString::empty());
		}

		if string.chars().take(1).last().unwrap().is_digit(10) {
			Ok(VersionString {
				interpreter: None,
				version: Some(Version::from_string(string)?),
			})
		}
		else {
			match string.find('-').and_then(|x| Some(string.split_at(x))) {
				Some(x) => {
					let version_str: String = x.1.chars().skip(1).collect();

					Ok(VersionString {
						interpreter: Some(String::from(x.0)),
						version: Some(Version::from_string(&version_str)?)
					})
				},
				None => Ok(VersionString {
					interpreter: Some(string.to_string()),
					version: None
				})
			}
		}
	}
}

#[cfg(test)]
mod version_tests {
	use super::*;

	#[test]
	fn from_string_with_all_properties() {
		let version = Version::from_string("3.2.0-preview2").unwrap();
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(Some(0), version.patch);
		assert_eq!(Some(String::from("preview2")), version.label);
	}

	#[test]
	fn from_string_with_major_version() {
		let version = Version::from_string("3").unwrap();
		assert_eq!(Some(3), version.major);
		assert_eq!(None, version.minor);
		assert_eq!(None, version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_major_and_minor_version() {
		let version = Version::from_string("3.20").unwrap();
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(20), version.minor);
		assert_eq!(None, version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_major_minor_and_patch() {
		let version = Version::from_string("1.22.5").unwrap();
		assert_eq!(Some(1), version.major);
		assert_eq!(Some(22), version.minor);
		assert_eq!(Some(5), version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_empty_string() {
		let version = Version::from_string("").unwrap();
		assert_eq!(None, version.major);
		assert_eq!(None, version.minor);
		assert_eq!(None, version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_hot_garbage() {
		let version = Version::from_string(".hgvgvt.eger-vfdvfd");
		assert!(version.is_err());
	}
}

#[cfg(test)]
mod version_string_tests {
	use super::*;

	#[test]
	fn from_string_with_all_properties() {
		let version_string = VersionString::from_string("ruby-3.2.0-preview2").unwrap();
		let version = version_string.version.unwrap();
		assert_eq!(Some(String::from("ruby")), version_string.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(Some(0), version.patch);
		assert_eq!(Some(String::from("preview2")), version.label);
	}

	#[test]
	fn from_string_with_interpreter() {
		let version_string = VersionString::from_string("ruby").unwrap();
		assert_eq!(Some(String::from("ruby")), version_string.interpreter);
		assert_eq!(None, version_string.version);
	}

	#[test]
	fn from_string_with_version() {
		let version_string = VersionString::from_string("3.2.0-preview2").unwrap();
		let version = version_string.version.unwrap();
		assert_eq!(None, version_string.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(Some(0), version.patch);
		assert_eq!(Some(String::from("preview2")), version.label);
	}

	#[test]
	fn from_string_with_major_minor_patch_and_label() {
		let version_string = VersionString::from_string("3.2.0-preview2").unwrap();
		let version = version_string.version.unwrap();
		assert_eq!(None, version_string.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(Some(0), version.patch);
		assert_eq!(Some(String::from("preview2")), version.label);
	}

	#[test]
	fn from_string_with_major_minor_and_patch() {
		let version_string = VersionString::from_string("3.2.0").unwrap();
		let version = version_string.version.unwrap();
		assert_eq!(None, version_string.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(Some(0), version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_major_and_minor() {
		let version_string = VersionString::from_string("3.2").unwrap();
		let version = version_string.version.unwrap();
		assert_eq!(None, version_string.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(None, version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_major() {
		let version_string = VersionString::from_string("3").unwrap();
		let version = version_string.version.unwrap();
		assert_eq!(None, version_string.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(None, version.minor);
		assert_eq!(None, version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_empty_string() {
		let version_string = VersionString::from_string("").unwrap();
		assert_eq!(None, version_string.interpreter);
		assert_eq!(None, version_string.version);
	}

	#[test]
	fn from_string_with_hot_garbage() {
		let version = VersionString::from_string(".hgvgvt.eger-vfdvfd");
		assert!(version.is_err());
	}
}
