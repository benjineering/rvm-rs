use crate::version::Version;
use std::fmt;
use std::cmp;

#[derive(Debug)]
#[derive(Eq)]
pub struct VersionId {
	pub interpreter: Option<String>,
	pub version: Option<Version>,
}

impl VersionId {
	pub fn interpreter_and_label(interpreter: &str, major: u8, minor: u8, patch: u8, label: &str) -> VersionId {
		VersionId {
			interpreter: Some(interpreter.to_string()),
			version: Some(Version::label(major, minor, patch, label)),
		}
	}

	pub fn interpreter(interpreter: &str) -> VersionId {
		VersionId {
			interpreter: Some(interpreter.to_string()),
			version: None,
		}
	}
	
	pub fn label(major: u8, minor: u8, patch: u8, label: &str) -> VersionId {
		VersionId {
			interpreter: None,
			version: Some(Version::label(major, minor, patch, label)),
		}
	}
	
	pub fn patch(major: u8, minor: u8, patch: u8) -> VersionId {
		VersionId {
			interpreter: None,
			version: Some(Version::patch(major, minor, patch)),
		}
	}
	
	pub fn minor(major: u8, minor: u8) -> VersionId {
		VersionId {
			interpreter: None,
			version: Some(Version::minor(major, minor)),
		}
	}
	
	pub fn major(major: u8) -> VersionId {
		VersionId {
			interpreter: None,
			version: Some(Version::major(major)),
		}
	}

	pub fn empty() -> VersionId {
		VersionId {
			interpreter: None,
			version: None,
		}
	}

	pub fn from_string(string: &str) -> Result<VersionId, ()> {
		if string.is_empty() { 
			return Ok(VersionId::empty());
		}

		if string.chars().take(1).last().unwrap().is_digit(10) {
			Ok(VersionId {
				interpreter: None,
				version: Some(Version::from_string(string)?),
			})
		}
		else {
			match string.find('-').and_then(|x| Some(string.split_at(x))) {
				Some(x) => {
					let version_str: String = x.1.chars().skip(1).collect();

					Ok(VersionId {
						interpreter: Some(String::from(x.0)),
						version: Some(Version::from_string(&version_str)?)
					})
				},
				None => Ok(VersionId {
					interpreter: Some(string.to_string()),
					version: None
				})
			}
		}
	}

	pub fn from_github_asset_name(asset_name: &str) -> Result<VersionId, ()> {
		let version_str = asset_name.split('-').nth(1);

		if version_str.is_none() {
			return Err(());
		}

		let version = Version::from_string(version_str.unwrap())?;
		Ok(VersionId { interpreter: Some("ruby".to_string()), version: Some(version) })
	}
}

impl fmt::Display for VersionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut vec: Vec<String> = vec!(); // TODO: make this lighter

		if self.interpreter.is_some() { 
			vec.push(self.interpreter.as_ref().unwrap().to_string());
		}

		if self.version.is_some() { 
			vec.push(format!("{}", self.version.as_ref().unwrap()));
		}		
        
        write!(f, "{}", vec.join("-"))
    }
}

impl cmp::Ord for VersionId {
    fn cmp(&self, other: &Self) -> cmp::Ordering {

		if self.interpreter == other.interpreter && self.version == other.version {
			return cmp::Ordering::Equal;
		}

		if self.interpreter == other.interpreter {
			return self.version.cmp(&other.version);
		}

        self.interpreter.cmp(&other.interpreter)
    }
}

impl cmp::PartialOrd for VersionId {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for VersionId {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

#[cfg(test)]
mod version_id_tests {
	use super::*;

	#[test]
	fn from_string_with_all_properties() {
		let version_id = VersionId::from_string("ruby-3.2.0-preview2").unwrap();
		let version = version_id.version.unwrap();
		assert_eq!(Some(String::from("ruby")), version_id.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(Some(0), version.patch);
		assert_eq!(Some(String::from("preview2")), version.label);
	}

	#[test]
	fn from_string_with_interpreter() {
		let version_id = VersionId::from_string("ruby").unwrap();
		assert_eq!(Some(String::from("ruby")), version_id.interpreter);
		assert_eq!(None, version_id.version);
	}

	#[test]
	fn from_string_with_version() {
		let version_id = VersionId::from_string("3.2.0-preview2").unwrap();
		let version = version_id.version.unwrap();
		assert_eq!(None, version_id.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(Some(0), version.patch);
		assert_eq!(Some(String::from("preview2")), version.label);
	}

	#[test]
	fn from_string_with_major_minor_patch_and_label() {
		let version_id = VersionId::from_string("3.2.0-preview2").unwrap();
		let version = version_id.version.unwrap();
		assert_eq!(None, version_id.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(Some(0), version.patch);
		assert_eq!(Some(String::from("preview2")), version.label);
	}

	#[test]
	fn from_string_with_major_minor_and_patch() {
		let version_id = VersionId::from_string("3.2.0").unwrap();
		let version = version_id.version.unwrap();
		assert_eq!(None, version_id.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(Some(0), version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_major_and_minor() {
		let version_id = VersionId::from_string("3.2").unwrap();
		let version = version_id.version.unwrap();
		assert_eq!(None, version_id.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(Some(2), version.minor);
		assert_eq!(None, version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_major() {
		let version_id = VersionId::from_string("3").unwrap();
		let version = version_id.version.unwrap();
		assert_eq!(None, version_id.interpreter);
		assert_eq!(Some(3), version.major);
		assert_eq!(None, version.minor);
		assert_eq!(None, version.patch);
		assert_eq!(None, version.label);
	}

	#[test]
	fn from_string_with_empty_string() {
		let version_id = VersionId::from_string("").unwrap();
		assert_eq!(None, version_id.interpreter);
		assert_eq!(None, version_id.version);
	}

	#[test]
	fn from_string_with_hot_garbage() {
		let version_id = VersionId::from_string(".hgvgvt.eger-vfdvfd");
		assert!(version_id.is_err());
	}

	#[test]
	fn fmt_with_all_properties() {
		let version_id = VersionId::interpreter_and_label("ruby", 3, 2, 0, "blorp");
		let formatted = format!("{}", version_id);
		assert_eq!("ruby-3.2.0-blorp", formatted);
	}
	#[test]
	fn fmt_with_interpreter() {
		let version_id = VersionId::interpreter("ruby");
		let formatted = format!("{}", version_id);
		assert_eq!("ruby", formatted);
	}

	#[test]
	fn fmt_with_major_minor_patch_and_label() {
		let version_id = VersionId::label(3, 2, 0, "preview2");
		let formatted = format!("{}", version_id);
		assert_eq!("3.2.0-preview2", formatted);
	}

	#[test]
	fn fmt_with_major_minor_and_patch() {
		let version_id = VersionId::patch(3, 2, 0);
		let formatted = format!("{}", version_id);
		assert_eq!("3.2.0", formatted);
	}

	#[test]
	fn fmt_with_major_and_minor() {
		let version_id = VersionId::minor(3, 2);
		let formatted = format!("{}", version_id);
		assert_eq!("3.2", formatted);
	}

	#[test]
	fn fmt_with_major() {
		let version_id = VersionId::major(3);
		let formatted = format!("{}", version_id);
		assert_eq!("3", formatted);
	}
}
