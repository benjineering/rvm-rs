use std::fmt;
use std::cmp;
use std::string::ToString;

#[derive(Eq)]
#[derive(Debug)]
pub struct Version {
	pub major: Option<u8>,
	pub minor: Option<u8>,
	pub patch: Option<u8>,
	pub label: Option<String>,
}

impl Version {
	#[cfg(test)]
	pub fn major(major: u8) -> Version {
		Version { major: Some(major), minor: None, patch: None, label: None }
	}

	#[cfg(test)]
	pub fn minor(major: u8, minor: u8) -> Version {
		Version { major: Some(major), minor: Some(minor), patch: None, label: None }
	}

	#[cfg(test)]
	pub fn patch(major: u8, minor: u8, patch: u8) -> Version {
		Version { major: Some(major), minor: Some(minor), patch: Some(patch), label: None }
	}

	#[cfg(test)]
	pub fn label(major: u8, minor: u8, patch: u8, label: &str) -> Version {
		Version { major: Some(major), minor: Some(minor), patch: Some(patch), label: Some(label.to_string()) }
	}

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

	pub fn from_github_asset_name(asset_name: &str) -> Result<Version, ()> {
		let version_str = asset_name.split('-').nth(1);

		if version_str.is_none() {
			return Err(());
		}

		Version::from_string(version_str.unwrap())
	}

	pub fn includes(&self, other: &Self) -> bool {
		if self.major.is_none() {
			return true;
		}

		if self.major != other.major {
			return false;
		}

		if self.minor.is_some() && self.minor != other.minor {
			return false;
		}

		if self.patch.is_some() && self.patch != other.patch {
			return false;
		}

		if self.label.is_some() && self.label != other.label {
			return false;
		}

		true
	}

	fn parse_int(string: &str) -> Result<u8, ()> {
		string.parse::<u8>().or_else(|_| Err(()))
	}

	fn numbers_to_string_vec(&self) -> Vec<String> {
		let mut vec: Vec<String> = vec!();

		if self.major.is_some() { 
			vec.push(self.major.unwrap().to_string());
		}
		else {
			return vec;
		}

		if self.minor.is_some() { 
			vec.push(self.minor.unwrap().to_string());
		}
		else {
			return vec;
		}

		if self.patch.is_some() { 
			vec.push(self.patch.unwrap().to_string());
		}

		vec
	}
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let string_vec = self.numbers_to_string_vec();
		let label = if self.label.is_some() { format!("-{}", self.label.as_ref().unwrap()) } else { "".to_string() };
        write!(f, "{}{}", string_vec.join("."), label)
    }
}

impl cmp::Ord for Version {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.major != other.major {
			return self.major.cmp(&other.major);
		}
		
        if self.minor != other.minor {
			return self.minor.cmp(&other.minor);
		}
		
        if self.patch != other.patch {
			return self.patch.cmp(&other.patch);
		}

		self.label.cmp(&other.label)
    }
}

impl cmp::PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
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

	#[test]
	fn fmt_with_all_properties() {
		let version = Version::label(3, 2, 1, "blorp");
		let formatted = format!("{}", version);
		assert_eq!("3.2.1-blorp", formatted);
	}

	#[test]
	fn fmt_with_major_minor_and_patch() {
		let version = Version::patch(3, 22, 1);
		let formatted = format!("{}", version);
		assert_eq!("3.22.1", formatted);
	}

	#[test]
	fn fmt_with_major_and_minor() {
		let version = Version::minor(2, 2);
		let formatted = format!("{}", version);
		assert_eq!("2.2", formatted);
	}

	#[test]
	fn fmt_with_major() {
		let version = Version::major(1);
		let formatted = format!("{}", version);
		assert_eq!("1", formatted);
	}
}
